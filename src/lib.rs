pub mod chat;
pub mod client;
pub mod model_variants;
pub mod models;

pub mod text_completion;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug, thiserror::Error)]
pub enum GptError {
    #[error("OpenAI API error: {0}")]
    OpenaiApiError(String),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("Serde error: {0}")]
    SerdeError(#[from] SerdeError),

    // io Result error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
