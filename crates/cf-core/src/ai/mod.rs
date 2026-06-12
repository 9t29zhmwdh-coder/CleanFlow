pub mod claude;
pub mod ollama;
pub mod prompts;

pub use claude::ClaudeClassifier;
pub use ollama::OllamaClassifier;

use crate::models::{AiClassification, ScannedFile};

pub trait AiClassifier: Send + Sync {
    fn classify_batch(
        &self,
        files: &[&ScannedFile],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Option<AiClassification>>>> + Send;
}
