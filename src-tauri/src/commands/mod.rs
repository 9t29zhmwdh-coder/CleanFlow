pub mod cleanup;
pub mod organize;
pub mod rules;
pub mod scan;
pub mod undo;

use tauri::State;

use cf_core::models::{AiBackend, AiBackendStatus, AppSettings};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings> {
    state.store.get_settings().map_err(Into::into)
}

#[tauri::command]
pub fn save_settings(settings: AppSettings, state: State<'_, AppState>) -> Result<()> {
    state.store.save_settings(&settings).map_err(Into::into)
}

#[tauri::command]
pub async fn check_ai_backend(state: State<'_, AppState>) -> Result<AiBackendStatus> {
    let settings = state.store.get_settings().unwrap_or_default();
    let claude_key = load_api_key("claude");
    let claude_available = !claude_key.is_empty();

    let ollama_url = &settings.ollama_url;
    let ollama_available = reqwest::Client::new()
        .get(format!("{ollama_url}/api/tags"))
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let active_backend = if claude_available {
        AiBackend::Claude
    } else if ollama_available {
        AiBackend::Ollama
    } else {
        AiBackend::RuleBasedOnly
    };

    Ok(AiBackendStatus {
        claude_available,
        ollama_available,
        active_backend,
    })
}

#[tauri::command]
pub fn open_in_finder(path: String) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| crate::error::CfError::Io(e.to_string()))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| crate::error::CfError::Io(e.to_string()))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&std::path::Path::new(&path).parent().unwrap_or_else(|| std::path::Path::new("/")))
            .spawn()
            .map_err(|e| crate::error::CfError::Io(e.to_string()))?;
    }
    Ok(())
}

#[tauri::command]
pub fn save_api_key(service: String, key: String) -> Result<()> {
    keyring::Entry::new("cleanflow", &service)
        .map_err(|e| crate::error::CfError::Other(e.to_string()))?
        .set_password(&key)
        .map_err(|e| crate::error::CfError::Other(e.to_string()))
}

#[tauri::command]
pub fn has_api_key(service: String) -> bool {
    keyring::Entry::new("cleanflow", &service)
        .map(|e| e.get_password().is_ok())
        .unwrap_or(false)
}

fn load_api_key(service: &str) -> String {
    keyring::Entry::new("cleanflow", service)
        .and_then(|e| e.get_password())
        .unwrap_or_default()
}
