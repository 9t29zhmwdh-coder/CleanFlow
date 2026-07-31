use std::path::PathBuf;
use std::time::Instant;

use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

use cf_core::{
    ai::{ClaudeClassifier, OllamaClassifier},
    models::{AiBackend, AppSettings, ScanPhase, ScanStatus, ScannedFile},
    Scanner, ScanOptions,
};
use crate::{error::Result, state::{AppState, ScanSession}};

/// Wie viele Dateien pro Anfrage. Der Prompt ist auf Stapel dieser Groesse
/// ausgelegt, siehe `cf_core::ai::prompts::batch_classify_prompt`.
const BATCH: usize = 50;

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
        let files = scanner.analyze_files(paths, &opts, |n| {
            emit_status(&app_clone, &scan_id_clone, ScanPhase::Analyzing, found, n, start.elapsed().as_millis() as u64);
        });

        let analyzed = files.len();

        // Klassifizierung. Die Verbraucherseite, also Planer und Regel-Engine,
        // liest `ai_classification` seit v0.1.0; gefuellt wurde das Feld nie,
        // weil dieser Aufruf fehlte.
        let settings = app_clone
            .try_state::<AppState>()
            .and_then(|s| s.store.get_settings().ok())
            .unwrap_or_default();

        let mut files = files;
        let requests = classify_files(&mut files, &settings, |done, total| {
            emit_progress(
                &app_clone,
                &scan_id_clone,
                ScanPhase::AiClassifying,
                found,
                analyzed,
                done,
                start.elapsed().as_millis() as u64,
            );
            let _ = total;
        })
        .await;

        let elapsed_ms = start.elapsed().as_millis() as u64;

        if let Some(state) = app_clone.try_state::<AppState>() {
            if let Some(session) = state.scans.lock().unwrap().get_mut(&scan_id_clone) {
                session.files = files;
            }
        }

        emit_progress(&app_clone, &scan_id_clone, ScanPhase::Done, found, analyzed, requests, elapsed_ms);
    });

    Ok(scan_id)
}

#[tauri::command]
pub fn get_scan_status(scan_id: String, state: State<'_, AppState>) -> Result<ScanStatus> {
    let scans = state.scans.lock().unwrap();
    scans
        .get(&scan_id)
        .map(|s| s.status.clone())
        .ok_or(crate::error::CfError::ScanNotFound(scan_id))
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
    if let Some(state) = app.try_state::<AppState>() {
        if let Some(session) = state.scans.lock().unwrap().get_mut(scan_id) {
            session.status = status.clone();
        }
    }
    let _ = app.emit(&format!("scan://status/{scan_id}"), &status);
}

/// Fuellt `ai_classification` fuer alle gescannten Dateien und liefert die
/// Anzahl abgesetzter Anfragen zurueck.
///
/// Ein Fehlschlag beendet den Scan nicht. Wer klassifizieren wollte und dessen
/// Ollama-Instanz nicht laeuft, soll trotzdem seinen regelbasierten Plan
/// bekommen, nicht eine leere Ansicht.
async fn classify_files(
    files: &mut [ScannedFile],
    settings: &AppSettings,
    mut on_progress: impl FnMut(usize, usize),
) -> usize {
    if settings.ai_backend == AiBackend::RuleBasedOnly || files.is_empty() {
        return 0;
    }

    let total = files.len().div_ceil(BATCH);
    let mut requests = 0usize;

    for start in (0..files.len()).step_by(BATCH) {
        let end = (start + BATCH).min(files.len());
        let batch: Vec<&ScannedFile> = files[start..end].iter().collect();

        let result = match settings.ai_backend {
            AiBackend::Ollama => {
                OllamaClassifier::new(settings.ollama_url.clone(), settings.ollama_model.clone())
                    .classify_batch(&batch)
                    .await
            }
            AiBackend::Claude => {
                let key = crate::commands::load_api_key("claude");
                if key.is_empty() {
                    return requests;
                }
                ClaudeClassifier::new(key).classify_batch(&batch).await
            }
            AiBackend::RuleBasedOnly => unreachable!("oben abgefangen"),
        };

        requests += 1;
        on_progress(requests, total);

        match result {
            Ok(classifications) => {
                for (file, cls) in files[start..end].iter_mut().zip(classifications) {
                    file.ai_classification = cls;
                }
            }
            // Der Rest des Stapels wird nicht versucht: faellt eine Anfrage
            // aus, faellt in aller Regel die naechste auch aus, und jede
            // kostet Zeit oder Geld.
            Err(_) => return requests,
        }
    }

    requests
}

fn emit_progress(
    app: &AppHandle,
    scan_id: &str,
    phase: ScanPhase,
    found: usize,
    analyzed: usize,
    ai_requests_made: usize,
    elapsed_ms: u64,
) {
    let status = ScanStatus {
        scan_id: scan_id.to_string(),
        phase,
        files_found: found,
        files_analyzed: analyzed,
        ai_requests_made,
        elapsed_ms,
        errors: vec![],
    };
    if let Some(state) = app.try_state::<AppState>() {
        if let Some(session) = state.scans.lock().unwrap().get_mut(scan_id) {
            session.status = status.clone();
        }
    }
    let _ = app.emit(&format!("scan://status/{scan_id}"), &status);
}
