use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

use cf_core::{
    models::{ScanPhase, ScanStatus},
    Scanner, ScanOptions,
};
use crate::{error::Result, state::{AppState, ScanSession}};

#[derive(serde::Deserialize)]
pub struct ScanOpts {
    pub follow_links: Option<bool>,
    pub max_depth: Option<usize>,
    pub skip_hidden: Option<bool>,
}

#[tauri::command]
pub async fn scan_directory(
    path: String,
    options: Option<ScanOpts>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String> {
    let scan_id = Uuid::new_v4().to_string();
    let root = PathBuf::from(&path);
    let sid = scan_id.clone();
    let opts = ScanOptions {
        follow_links: options.as_ref().and_then(|o| o.follow_links).unwrap_or(false),
        max_depth: options.as_ref().and_then(|o| o.max_depth),
        skip_hidden: options.as_ref().and_then(|o| o.skip_hidden).unwrap_or(true),
        ..Default::default()
    };

    // Initial status
    {
        let mut scans = state.scans.lock().unwrap();
        let status = ScanStatus {
            scan_id: scan_id.clone(),
            phase: ScanPhase::Walking,
            files_found: 0,
            files_analyzed: 0,
            ai_requests_made: 0,
            elapsed_ms: 0,
            errors: vec![],
        };
        scans.insert(scan_id.clone(), ScanSession {
            status: status.clone(),
            files: vec![],
            plan: None,
        });
        let _ = app.emit(&format!("scan://status/{}", scan_id), &status);
    }

    let scan_id_clone = scan_id.clone();
    let app_clone = app.clone();

    tokio::spawn(async move {
        let start = Instant::now();
        let scanner = Scanner::new();

        // Walk
        let paths = scanner.walk(&root, &opts);
        let found = paths.len();

        emit_status(&app_clone, &scan_id_clone, ScanPhase::Analyzing, found, 0, start.elapsed().as_millis() as u64);

        // Analyze
        let mut files = scanner.analyze_files(paths, &opts, |n| {
            emit_status(&app_clone, &scan_id_clone, ScanPhase::Analyzing, found, n, start.elapsed().as_millis() as u64);
        });

        emit_status(&app_clone, &scan_id_clone, ScanPhase::Done, found, files.len(), start.elapsed().as_millis() as u64);
    });

    Ok(scan_id)
}

#[tauri::command]
pub fn get_scan_status(scan_id: String, state: State<'_, AppState>) -> Result<ScanStatus> {
    let scans = state.scans.lock().unwrap();
    scans
        .get(&scan_id)
        .map(|s| s.status.clone())
        .ok_or_else(|| crate::error::CfError::ScanNotFound(scan_id))
}

#[tauri::command]
pub fn get_scanned_files(scan_id: String, state: State<'_, AppState>) -> Result<Vec<cf_core::models::ScannedFile>> {
    let scans = state.scans.lock().unwrap();
    Ok(scans.get(&scan_id).map(|s| s.files.clone()).unwrap_or_default())
}

#[tauri::command]
pub fn cancel_scan(scan_id: String, state: State<'_, AppState>) -> Result<()> {
    let mut scans = state.scans.lock().unwrap();
    if let Some(session) = scans.get_mut(&scan_id) {
        session.status.phase = ScanPhase::Cancelled;
    }
    Ok(())
}

fn emit_status(app: &AppHandle, scan_id: &str, phase: ScanPhase, found: usize, analyzed: usize, elapsed_ms: u64) {
    let status = ScanStatus {
        scan_id: scan_id.to_string(),
        phase,
        files_found: found,
        files_analyzed: analyzed,
        ai_requests_made: 0,
        elapsed_ms,
        errors: vec![],
    };
    let _ = app.emit(&format!("scan://status/{scan_id}"), &status);
}
