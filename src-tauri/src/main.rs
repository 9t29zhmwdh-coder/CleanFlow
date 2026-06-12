// Tauri v2 entry point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod state;

use std::path::PathBuf;
use state::AppState;
use commands::{
    check_ai_backend, get_settings, has_api_key, open_in_finder, save_api_key, save_settings,
    cleanup::{find_duplicates, find_junk, find_old_versions},
    organize::{execute_cleanflow, execute_plan, preview_plan},
    rules::{delete_rule, list_rules, save_rule, test_rule},
    scan::{cancel_scan, get_scan_status, get_scanned_files, scan_directory},
    undo::{list_history, undo_by_id, undo_last},
};

fn main() {
    let data_dir = app_data_dir();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState::new(data_dir).expect("Failed to initialise app state"))
        .invoke_handler(tauri::generate_handler![
            // Scan
            scan_directory,
            get_scan_status,
            get_scanned_files,
            cancel_scan,
            // Organize
            preview_plan,
            execute_plan,
            execute_cleanflow,
            // Rules
            list_rules,
            save_rule,
            delete_rule,
            test_rule,
            // Cleanup
            find_duplicates,
            find_junk,
            find_old_versions,
            // Undo
            list_history,
            undo_last,
            undo_by_id,
            // System
            get_settings,
            save_settings,
            check_ai_backend,
            open_in_finder,
            save_api_key,
            has_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn app_data_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    home.join(".cleanflow")
}
