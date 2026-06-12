use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::{AiClassification, AiSource, ScannedFile};
use super::prompts::batch_classify_prompt;

pub struct OllamaClassifier {
    base_url: String,
    model: String,
    client: Client,
}

impl OllamaClassifier {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            model,
            client: Client::new(),
        }
    }

    pub async fn classify_batch(
        &self,
        files: &[&ScannedFile],
    ) -> anyhow::Result<Vec<Option<AiClassification>>> {
        let (system_prompt, user_content) = batch_classify_prompt(files);

        let request = OllamaRequest {
            model: self.model.clone(),
            messages: vec![
                OllamaMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                OllamaMessage {
                    role: "user".to_string(),
                    content: user_content,
                },
            ],
            stream: false,
        };

        let url = format!("{}/api/chat", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Ollama API error {status}");
        }

        let resp: OllamaResponse = response.json().await?;
        let text = resp.message.content;

        // Reuse the same parser as Claude (same prompt → same JSON format)
        super::claude::parse_batch_response_pub(&text, files)
    }

    pub async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
}
