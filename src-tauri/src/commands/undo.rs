use tauri::State;

use cf_core::{executor::UndoResult, models::HistoryEntry, Executor};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub fn list_history(limit: usize, state: State<'_, AppState>) -> Result<Vec<HistoryEntry>> {
    let entries = state.journal.list(limit)?;
    Ok(entries)
}

#[tauri::command]
pub fn undo_last(state: State<'_, AppState>) -> Result<UndoResult> {
    let executor = Executor::new(state.journal.as_ref().clone());
    executor.undo_last().map_err(|e| crate::error::CfError::Other(e.to_string()))
}

#[tauri::command]
pub fn undo_by_id(history_id: String, state: State<'_, AppState>) -> Result<UndoResult> {
    let executor = Executor::new(state.journal.as_ref().clone());
    executor
        .undo_by_id(&history_id)
        .map_err(|e| crate::error::CfError::Other(e.to_string()))
}
