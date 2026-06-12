use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::models::{AiClassification, AiSource, FileCategory, ScannedFile};
use super::prompts::batch_classify_prompt;

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-haiku-4-5-20251001";
const BATCH_SIZE: usize = 50;

pub struct ClaudeClassifier {
    api_key: String,
    client: Client,
}

impl ClaudeClassifier {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn classify_all(
        &self,
        files: &[ScannedFile],
        progress_tx: Option<mpsc::Sender<usize>>,
    ) -> anyhow::Result<Vec<(String, AiClassification)>> {
        let mut results = vec![];

        for (batch_idx, chunk) in files.chunks(BATCH_SIZE).enumerate() {
            let refs: Vec<&ScannedFile> = chunk.iter().collect();
            let batch_result = self.classify_batch(&refs).await?;

            for (file, classification) in chunk.iter().zip(batch_result.into_iter()) {
                if let Some(c) = classification {
                    results.push((file.id.clone(), c));
                }
            }

            if let Some(tx) = &progress_tx {
                let _ = tx.send((batch_idx + 1) * BATCH_SIZE).await;
            }
        }

        Ok(results)
    }

    pub async fn classify_batch(
        &self,
        files: &[&ScannedFile],
    ) -> anyhow::Result<Vec<Option<AiClassification>>> {
        let (system_prompt, user_content) = batch_classify_prompt(files);

        let request = ClaudeRequest {
            model: MODEL.to_string(),
            max_tokens: 2048,
            system: Some(system_prompt),
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: user_content,
            }],
        };

        let response = self
            .client
            .post(API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Claude API error {status}: {body}");
        }

        let claude_resp: ClaudeResponse = response.json().await?;
        let text = claude_resp
            .content
            .iter()
            .filter(|b| b.block_type == "text")
            .map(|b| b.text.as_deref().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("");

        parse_batch_response(&text, files)
    }
}

pub fn parse_batch_response_pub(
    text: &str,
    files: &[&ScannedFile],
) -> anyhow::Result<Vec<Option<AiClassification>>> {
    parse_batch_response(text, files)
}

fn parse_batch_response(
    text: &str,
    files: &[&ScannedFile],
) -> anyhow::Result<Vec<Option<AiClassification>>> {
    // Strip potential markdown code fences
    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let raw: Vec<serde_json::Value> = serde_json::from_str(json_str)
        .unwrap_or_else(|_| vec![]);

    let mut out: Vec<Option<AiClassification>> = vec![None; files.len()];

    for item in &raw {
        let id = item["id"].as_str().unwrap_or("");
        if let Some(idx) = files.iter().position(|f| f.id == id) {
            let category = parse_category(item["category"].as_str().unwrap_or("Unknown"));
            let tags: Vec<String> = item["tags"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            let confidence = item["confidence"].as_f64().unwrap_or(0.5) as f32;
            let summary = item["summary"].as_str().map(|s| s.to_string());

            out[idx] = Some(AiClassification {
                category,
                confidence,
                tags,
                project: None,
                summary,
                source: AiSource::Claude,
            });
        }
    }

    Ok(out)
}

fn parse_category(s: &str) -> FileCategory {
    match s {
        "Invoice"     => FileCategory::Invoice,
        "Contract"    => FileCategory::Contract,
        "Receipt"     => FileCategory::Receipt,
        "Screenshot"  => FileCategory::Screenshot,
        "Photo"       => FileCategory::Photo,
        "Video"       => FileCategory::Video,
        "Audio"       => FileCategory::Audio,
        "Code"        => FileCategory::Code,
        "Document"    => FileCategory::Document,
        "Spreadsheet" => FileCategory::Spreadsheet,
        "Archive"     => FileCategory::Archive,
        "Installer"   => FileCategory::Installer,
        "Temporary"   => FileCategory::Temporary,
        other         => FileCategory::Unknown(other.to_string()),
    }
}

// ── API types ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<ClaudeMessage>,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}
