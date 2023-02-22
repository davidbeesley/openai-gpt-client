use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model_variants::ModelId;

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TextCompletionRequest {
    pub model: ModelId,
    pub prompt: String,
    pub suffix: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub logprobs: Option<i32>,
    pub echo: Option<bool>,
    pub stop: Option<Value>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub best_of: Option<i32>,
    pub logit_bias: Option<Value>,
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextCompletionResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i32>,
    pub model: Option<String>,
    pub choices: Option<Vec<TextCompletionChoice>>,
    pub usage: Option<TextCompletionUsage>,
}

impl Display for TextCompletionResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(choices) = &self.choices {
            if let Some(text) = choices.get(0) {
                write!(f, "{}", text.text)
            } else {
                write!(f, "No response")
            }
        } else {
            write!(f, "No response")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextCompletionChoice {
    pub text: String,
    pub index: i32,
    pub logprobs: Option<TextCompletionLogprobs>,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextCompletionLogprobs {
    // Add any fields you need here
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextCompletionUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}
