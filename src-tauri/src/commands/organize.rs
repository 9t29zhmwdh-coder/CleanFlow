use tauri::State;

use cf_core::{
    builtin_rules,
    executor::{ExecutionResult, UndoResult},
    models::OrganizePlan,
    Executor, Planner, RuleEngine,
};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub fn preview_plan(scan_id: String, state: State<'_, AppState>) -> Result<OrganizePlan> {
    let mut scans = state.scans.lock().unwrap();
    let session = scans
        .get_mut(&scan_id)
        .ok_or_else(|| crate::error::CfError::ScanNotFound(scan_id.clone()))?;

    let user_rules = state.store.list_rules().unwrap_or_default();
    let rules = if user_rules.is_empty() {
        builtin_rules()
    } else {
        [builtin_rules(), user_rules].concat()
    };

    let engine = RuleEngine::new(rules);
    let planner = Planner::new(&engine);
    let plan = planner.build_plan(
        session.status.scan_id.clone().into(),
        &mut session.files,
    );

    let result = plan.clone();
    session.plan = Some(plan);
    Ok(result)
}

#[tauri::command]
pub fn execute_plan(
    plan_id: String,
    selected_ids: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<ExecutionResult> {
    let scans = state.scans.lock().unwrap();

    let plan = scans
        .values()
        .find_map(|s| s.plan.as_ref().filter(|p| p.id == plan_id))
        .ok_or_else(|| crate::error::CfError::Other(format!("Plan not found: {plan_id}")))?
        .clone();

    drop(scans);

    let executor = Executor::new(
        cf_core::Journal::open(&state.data_dir.join("journal"))
            .map_err(|e| crate::error::CfError::Other(e.to_string()))?,
    );
    let result = executor
        .execute_plan(&plan, selected_ids.as_deref())
        .map_err(|e| crate::error::CfError::Other(e.to_string()))?;

    Ok(result)
}

#[tauri::command]
pub fn execute_cleanflow(scan_id: String, state: State<'_, AppState>) -> Result<ExecutionResult> {
    // Build plan then execute all selected actions
    let plan = preview_plan(scan_id, state.clone())?;
    execute_plan(plan.id, None, state)
}
