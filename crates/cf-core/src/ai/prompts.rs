use crate::models::ScannedFile;

/// Build a single batch prompt for up to 50 files.
/// Returns the system prompt and user message as (system, user).
pub fn batch_classify_prompt(files: &[&ScannedFile]) -> (String, String) {
    let system = r#"You are a file classification assistant. Given a list of files with their metadata, classify each file and return a JSON array.

For each file return an object with these fields:
- "id": the file id (string, unchanged from input)
- "category": one of: Invoice, Contract, Receipt, Screenshot, Photo, Video, Audio, Code, Document, Spreadsheet, Archive, Installer, Temporary, Unknown
- "tags": array of lowercase descriptive tags (max 5), e.g. ["finance", "2024", "work"]
- "confidence": float 0.0–1.0
- "summary": one-sentence description (optional, null if unclear)

Return ONLY a valid JSON array, no markdown, no explanation."#.to_string();

    let file_list: Vec<serde_json::Value> = files
        .iter()
        .map(|f| {
            serde_json::json!({
                "id":    f.id,
                "name":  f.path.file_name().unwrap_or_default().to_string_lossy(),
                "size":  f.size_bytes,
                "mime":  f.mime_type,
            })
        })
        .collect();

    let user = serde_json::to_string_pretty(&file_list).unwrap_or_default();
    (system, user)
}
