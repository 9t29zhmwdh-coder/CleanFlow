use std::path::PathBuf;

use chrono::Utc;
use uuid::Uuid;

use crate::models::{
    Action, ActionReason, DuplicateGroup, JunkFile, KeepStrategy, OrganizePlan,
    PlannedAction, PlanStats, ScannedFile,
};
use crate::rules::RuleEngine;
use crate::scanner::find_duplicates;

pub struct Planner<'a> {
    rule_engine: &'a RuleEngine,
}

impl<'a> Planner<'a> {
    pub fn new(rule_engine: &'a RuleEngine) -> Self {
        Self { rule_engine }
    }

    pub fn build_plan(
        &self,
        source_directory: PathBuf,
        files: &mut Vec<ScannedFile>,
    ) -> OrganizePlan {
        let mut all_actions: Vec<PlannedAction> = vec![];
        let mut stats = PlanStats::default();

        // 1. Rule-based actions
        for file in files.iter() {
            let rule_actions = self.rule_engine.apply(file);
            stats.rule_matches += rule_actions.len();
            all_actions.extend(rule_actions);
        }

        // 2. AI-suggestion actions (from classification)
        for file in files.iter() {
            if let Some(ref cls) = file.ai_classification {
                if let Some(action) = ai_suggestion_action(file, cls) {
                    // Only suggest if not already covered by a rule
                    let already_planned = all_actions
                        .iter()
                        .any(|pa| matches!(&pa.action, Action::Move { file_id, .. } if file_id == &file.id));
                    if !already_planned {
                        all_actions.push(action);
                        stats.ai_suggestions += 1;
                    }
                }
            }
        }

        // 3. Duplicate actions
        let dup_groups = find_duplicates(files);
        for group in &dup_groups {
            let actions = duplicate_group_actions(group);
            stats.duplicates_found += group.files.len().saturating_sub(1);
            stats.bytes_freed += group.total_wasted_bytes;
            all_actions.extend(actions);
        }

        // 4. Junk actions
        let junk: Vec<&ScannedFile> = files.iter().filter(|f| f.flags.is_junk).collect();
        for file in &junk {
            stats.junk_found += 1;
            all_actions.push(PlannedAction {
                id: Uuid::new_v4().to_string(),
                action: Action::Trash {
                    file_id: file.id.clone(),
                    path: file.path.clone(),
                },
                reason: ActionReason::JunkDetected {
                    junk_type: junk_type_label(file),
                },
                priority: 70,
                selected: true,
            });
        }

        // 5. Old version actions
        for file in files.iter().filter(|f| f.flags.is_old_version) {
            all_actions.push(PlannedAction {
                id: Uuid::new_v4().to_string(),
                action: Action::Trash {
                    file_id: file.id.clone(),
                    path: file.path.clone(),
                },
                reason: ActionReason::OldVersion {
                    canonical_path: file.path.clone(),
                },
                priority: 40,
                selected: false, // Off by default — user decides
            });
        }

        // Deduplicate: keep highest-priority action per file
        all_actions.sort_by(|a, b| b.priority.cmp(&a.priority));
        all_actions.dedup_by_key(|a| action_file_id(&a.action));

        stats.files_affected = all_actions
            .iter()
            .filter(|pa| !matches!(pa.action, Action::CreateDirectory { .. }))
            .count();

        OrganizePlan {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().timestamp(),
            source_directory,
            actions: all_actions,
            stats,
        }
    }
}

fn ai_suggestion_action(
    file: &ScannedFile,
    cls: &crate::models::AiClassification,
) -> Option<PlannedAction> {
    let dest = category_destination(&cls.category)?;
    let filename = file.path.file_name()?;
    Some(PlannedAction {
        id: Uuid::new_v4().to_string(),
        action: Action::Move {
            file_id: file.id.clone(),
            from: file.path.clone(),
            to: dest.join(filename),
        },
        reason: ActionReason::AiSuggestion {
            explanation: format!(
                "Classified as {} (confidence {:.0}%)",
                cls.category,
                cls.confidence * 100.0
            ),
        },
        priority: (cls.confidence * 50.0) as u8,
        selected: cls.confidence > 0.7,
    })
}

fn category_destination(cat: &crate::models::FileCategory) -> Option<PathBuf> {
    use crate::models::FileCategory::*;
    let home = dirs_home();
    let path = match cat {
        Invoice | Receipt  => home.join("Documents/Finance"),
        Contract           => home.join("Documents/Contracts"),
        Screenshot         => home.join("Pictures/Screenshots"),
        Photo              => home.join("Pictures"),
        Video              => home.join("Movies"),
        Audio              => home.join("Music"),
        Code               => home.join("Developer"),
        Document           => home.join("Documents"),
        Spreadsheet        => home.join("Documents/Spreadsheets"),
        Archive | Installer=> home.join("Documents/Archive"),
        Temporary          => return None,
        Unknown(_)         => return None,
    };
    Some(path)
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("~"))
}

fn duplicate_group_actions(group: &DuplicateGroup) -> Vec<PlannedAction> {
    // Keep newest, trash the rest
    let mut sorted = group.files.clone();
    sorted.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    sorted
        .iter()
        .skip(1)
        .map(|f| PlannedAction {
            id: Uuid::new_v4().to_string(),
            action: Action::Trash {
                file_id: f.id.clone(),
                path: f.path.clone(),
            },
            reason: ActionReason::DuplicateGroup {
                group_id: group.group_id.clone(),
                keep_strategy: KeepStrategy::Newest,
            },
            priority: 80,
            selected: true,
        })
        .collect()
}

fn junk_type_label(file: &ScannedFile) -> String {
    let ext = file
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "dmg" => "macOS Disk Image (installer)".into(),
        "tmp" => "Temporary file".into(),
        "log" => "Log file".into(),
        "cache" => "Cache file".into(),
        _ => "System junk file".into(),
    }
}

fn action_file_id(action: &Action) -> String {
    match action {
        Action::Move   { file_id, .. } => file_id.clone(),
        Action::Rename { file_id, .. } => file_id.clone(),
        Action::Trash  { file_id, .. } => file_id.clone(),
        Action::Tag    { file_id, .. } => file_id.clone(),
        Action::CreateDirectory { path } => path.to_string_lossy().to_string(),
    }
}
