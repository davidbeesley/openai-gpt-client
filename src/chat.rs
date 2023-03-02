use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{client::Stop, model_variants::ModelId};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct ChatRequest {
    pub model: ModelId,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub stop: Option<Stop>,
    pub max_tokens: Option<i32>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub logit_bias: Option<HashMap<String, f64>>,
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionUsage,
}
#[derive(Debug, Deserialize)]
pub struct ChatCompletionUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug)]
pub struct ChatHistory {
    stack: Vec<ChatMessage>,
}

impl ChatHistory {
    pub fn new() -> Self {
        ChatHistory { stack: vec![] }
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.stack.push(message);
    }

    pub fn pop_message(&mut self) -> Option<ChatMessage> {
        self.stack.pop()
    }

    pub fn clear_history(&mut self) {
        self.stack.clear();
    }

    pub fn get_history(&self) -> &[ChatMessage] {
        &self.stack
    }
}
