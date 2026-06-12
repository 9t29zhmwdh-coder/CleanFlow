use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use crate::models::{
    Action, ExecutedAction, HistoryEntry, OrganizePlan, PlannedAction, UndoData,
};
use crate::journal::Journal;

pub struct Executor {
    journal: Journal,
}

impl Executor {
    pub fn new(journal: Journal) -> Self {
        Self { journal }
    }

    pub fn execute_plan(
        &self,
        plan: &OrganizePlan,
        selected_ids: Option<&[String]>,
    ) -> anyhow::Result<ExecutionResult> {
        let actions: Vec<&PlannedAction> = plan
            .actions
            .iter()
            .filter(|pa| {
                pa.selected
                    && selected_ids
                        .map(|ids| ids.contains(&pa.id))
                        .unwrap_or(true)
            })
            .collect();

        let mut executed = vec![];
        let mut errors = vec![];

        for pa in actions {
            match self.execute_action(&pa.action) {
                Ok(undo) => executed.push(ExecutedAction {
                    action: pa.action.clone(),
                    undo_data: undo,
                }),
                Err(e) => errors.push(format!("{}: {e}", action_path(&pa.action))),
            }
        }

        let entry = HistoryEntry {
            id: Uuid::new_v4().to_string(),
            executed_at: Utc::now().timestamp(),
            actions: executed.clone(),
            plan_id: plan.id.clone(),
        };
        self.journal.push(&entry)?;

        Ok(ExecutionResult {
            history_id: entry.id,
            executed_count: executed.len(),
            error_count: errors.len(),
            errors,
        })
    }

    pub fn undo_last(&self) -> anyhow::Result<UndoResult> {
        let entry = self.journal.pop()?.ok_or_else(|| anyhow::anyhow!("Nothing to undo"))?;
        self.undo_entry(&entry)
    }

    pub fn undo_by_id(&self, history_id: &str) -> anyhow::Result<UndoResult> {
        let entry = self
            .journal
            .get(history_id)?
            .ok_or_else(|| anyhow::anyhow!("History entry not found: {history_id}"))?;
        self.undo_entry(&entry)
    }

    fn undo_entry(&self, entry: &HistoryEntry) -> anyhow::Result<UndoResult> {
        let mut undone = 0;
        let mut errors = vec![];

        for ea in entry.actions.iter().rev() {
            match undo_action(&ea.action, &ea.undo_data) {
                Ok(_)  => undone += 1,
                Err(e) => errors.push(e.to_string()),
            }
        }

        self.journal.remove(&entry.id)?;

        Ok(UndoResult {
            undone_count: undone,
            errors,
        })
    }

    fn execute_action(&self, action: &Action) -> anyhow::Result<UndoData> {
        match action {
            Action::Move { from, to, .. } => {
                if let Some(parent) = to.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::rename(from, to)?;
                Ok(UndoData::FileWasMoved {
                    original_path: from.clone(),
                })
            }
            Action::Rename { path, new_name, .. } => {
                let new_path = path.parent()
                    .map(|p| p.join(new_name))
                    .unwrap_or_else(|| Path::new(new_name).to_path_buf());
                let original_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                std::fs::rename(path, &new_path)?;
                Ok(UndoData::FileWasRenamed { original_name })
            }
            Action::Trash { path, .. } => {
                trash::delete(path)?;
                Ok(UndoData::FileWasTrashed {
                    original_path: path.clone(),
                })
            }
            Action::Tag { .. } => Ok(UndoData::NoUndo),
            Action::CreateDirectory { path } => {
                std::fs::create_dir_all(path)?;
                Ok(UndoData::DirectoryCreated { path: path.clone() })
            }
        }
    }
}

fn undo_action(action: &Action, undo_data: &UndoData) -> anyhow::Result<()> {
    match undo_data {
        UndoData::FileWasMoved { original_path } => {
            let current_path = match action {
                Action::Move { to, .. } => to,
                _ => anyhow::bail!("Undo data mismatch"),
            };
            if let Some(parent) = original_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::rename(current_path, original_path)?;
        }
        UndoData::FileWasTrashed { original_path } => {
            // Restore from trash not universally possible — warn
            tracing::warn!("Cannot restore from trash: {:?}", original_path);
        }
        UndoData::FileWasRenamed { original_name } => {
            let current_path = match action {
                Action::Rename { path, new_name, .. } => {
                    path.parent()
                        .map(|p| p.join(new_name))
                        .unwrap_or_else(|| Path::new(new_name).to_path_buf())
                }
                _ => anyhow::bail!("Undo data mismatch"),
            };
            let original_path = current_path.parent()
                .map(|p| p.join(original_name))
                .unwrap_or_else(|| Path::new(original_name).to_path_buf());
            std::fs::rename(&current_path, &original_path)?;
        }
        UndoData::DirectoryCreated { path } => {
            // Only remove if empty
            let _ = std::fs::remove_dir(path);
        }
        UndoData::NoUndo => {}
    }
    Ok(())
}

fn action_path(action: &Action) -> String {
    match action {
        Action::Move   { from, .. }  => from.display().to_string(),
        Action::Rename { path, .. }  => path.display().to_string(),
        Action::Trash  { path, .. }  => path.display().to_string(),
        Action::Tag    { file_id, .. }=> file_id.clone(),
        Action::CreateDirectory { path } => path.display().to_string(),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExecutionResult {
    pub history_id: String,
    pub executed_count: usize,
    pub error_count: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UndoResult {
    pub undone_count: usize,
    pub errors: Vec<String>,
}
