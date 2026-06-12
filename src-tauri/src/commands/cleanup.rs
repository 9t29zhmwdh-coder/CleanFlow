use tauri::State;

use cf_core::models::{DuplicateGroup, JunkFile, VersionGroup};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub fn find_duplicates(scan_id: String, state: State<'_, AppState>) -> Result<Vec<DuplicateGroup>> {
    let mut scans = state.scans.lock().unwrap();
    let session = scans
        .get_mut(&scan_id)
        .ok_or_else(|| crate::error::CfError::ScanNotFound(scan_id))?;

    let groups = cf_core::find_duplicates(&mut session.files);
    Ok(groups)
}

#[tauri::command]
pub fn find_junk(scan_id: String, state: State<'_, AppState>) -> Result<Vec<JunkFile>> {
    let scans = state.scans.lock().unwrap();
    let session = scans
        .get(&scan_id)
        .ok_or_else(|| crate::error::CfError::ScanNotFound(scan_id))?;

    let junk: Vec<JunkFile> = session
        .files
        .iter()
        .filter(|f| f.flags.is_junk)
        .map(|f| JunkFile {
            file: f.clone(),
            junk_type: junk_label(f),
            reason: junk_reason(f),
        })
        .collect();

    Ok(junk)
}

#[tauri::command]
pub fn find_old_versions(scan_id: String, state: State<'_, AppState>) -> Result<Vec<VersionGroup>> {
    use std::collections::HashMap;

    let scans = state.scans.lock().unwrap();
    let session = scans
        .get(&scan_id)
        .ok_or_else(|| crate::error::CfError::ScanNotFound(scan_id))?;

    let mut groups: HashMap<String, Vec<_>> = HashMap::new();
    for f in session.files.iter().filter(|f| f.flags.is_old_version) {
        let stem = f
            .path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        // Normalise: remove version suffixes
        let canonical = stem
            .replace(" copy", "")
            .replace("_backup", "")
            .replace("_old", "")
            .trim()
            .to_string();
        groups.entry(canonical).or_default().push(f.clone());
    }

    Ok(groups
        .into_iter()
        .map(|(name, versions)| VersionGroup {
            canonical_name: name,
            versions,
        })
        .collect())
}

fn junk_label(f: &cf_core::models::ScannedFile) -> String {
    let ext = f.path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    match ext.as_str() {
        "dmg" => "Disk Image".into(),
        "tmp" => "Temp File".into(),
        "log" => "Log File".into(),
        "cache" => "Cache File".into(),
        _ => "System Junk".into(),
    }
}

fn junk_reason(f: &cf_core::models::ScannedFile) -> String {
    let name = f.path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
    if name == ".ds_store" { return "macOS metadata file, safe to delete".into(); }
    format!("Detected as {} — typically safe to remove", junk_label(f))
}
