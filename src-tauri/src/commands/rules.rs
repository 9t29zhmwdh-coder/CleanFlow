use tauri::State;

use cf_core::models::{Rule, ScannedFile};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub fn list_rules(state: State<'_, AppState>) -> Result<Vec<Rule>> {
    let user_rules = state.store.list_rules()?;
    if user_rules.is_empty() {
        return Ok(cf_core::builtin_rules());
    }
    Ok([cf_core::builtin_rules(), user_rules].concat())
}

#[tauri::command]
pub fn save_rule(rule: Rule, state: State<'_, AppState>) -> Result<Rule> {
    state.store.save_rule(&rule)?;
    Ok(rule)
}

#[tauri::command]
pub fn delete_rule(rule_id: String, state: State<'_, AppState>) -> Result<()> {
    state.store.delete_rule(&rule_id)?;
    Ok(())
}

#[derive(serde::Serialize)]
pub struct RuleMatch {
    pub rule_id: String,
    pub rule_name: String,
    pub file_id: String,
    pub file_path: String,
}

#[tauri::command]
pub fn test_rule(rule: Rule, files: Vec<ScannedFile>, state: State<'_, AppState>) -> Result<Vec<RuleMatch>> {
    let engine = cf_core::RuleEngine::new(vec![rule.clone()]);
    let matches: Vec<RuleMatch> = files
        .iter()
        .flat_map(|f| {
            engine.apply(f).into_iter().map(|_| RuleMatch {
                rule_id: rule.id.clone(),
                rule_name: rule.name.clone(),
                file_id: f.id.clone(),
                file_path: f.path.display().to_string(),
            })
        })
        .collect();
    Ok(matches)
}
