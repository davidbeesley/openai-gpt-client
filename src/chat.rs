use std::collections::{HashMap, VecDeque};

use log::info;
use serde::{Deserialize, Serialize};

use crate::{client::Stop, model_variants::ModelId};

pub const MAX_LENGTH: usize = 6000;
pub const SUMMARIZE_LENGTH: usize = 2000;

// pub const MAX_LENGTH: usize = 600;
// pub const SUMMARIZE_LENGTH: usize = 200;
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

#[derive(Debug)]
pub struct ChatHistory {
    queue: VecDeque<ChatMessage>,
    prompt_message: Option<ChatMessage>,
    summary_messages: Vec<ChatMessage>,
}

impl ChatHistory {
    pub fn new(prompt_message: Option<String>) -> Self {
        ChatHistory {
            queue: Default::default(),
            prompt_message: prompt_message.map(|prompt| ChatMessage {
                role: Role::System,
                content: prompt,
            }),
            summary_messages: Default::default(),
        }
    }

    pub fn add_initial_messages(&mut self, messages: Vec<ChatMessage>) {
        self.queue.extend(messages)
    }

    pub fn summary_needed(&mut self, summary_prompt: ChatMessage) -> Option<Vec<ChatMessage>> {
        let total_length: usize = self.queue.iter().map(|message| message.content.len()).sum();
        if total_length < MAX_LENGTH - SUMMARIZE_LENGTH {
            return None;
        }
        let mut selected_messages = vec![];
        if let Some(prompt_message) = &self.prompt_message {
            selected_messages.push(prompt_message.clone());
        }
        selected_messages.extend(self.summary_messages.iter().cloned());
        let mut current_length = 0;
        while current_length < SUMMARIZE_LENGTH {
            if let Some(message) = self.queue.pop_front() {
                current_length += message.content.len();
                selected_messages.push(message);
            } else {
                break;
            }
        }
        selected_messages.push(summary_prompt);
        Some(selected_messages)
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.queue.push_back(message);
    }

    pub fn clear_history(&mut self) {
        self.queue.clear();
    }

    pub fn get_history(&self) -> Vec<ChatMessage> {
        let mut messages = vec![];
        if let Some(prompt_message) = &self.prompt_message {
            messages.push(prompt_message.clone());
        }
        messages.extend(self.summary_messages.iter().cloned());
        messages.extend(self.queue.iter().cloned());
        info!(
            "Length of history: {}",
            messages
                .iter()
                .map(|message| message.content.len())
                .sum::<usize>()
        );
        messages
    }

    pub fn add_summary(&mut self, summary_message: String) {
        self.summary_messages.push(ChatMessage {
            role: Role::System,
            content: format!("The conversation grew too big to keep in memory. This is a summary of part of the converstaion that was removed. All summary: {}", summary_message),
        });
        info!(
            "Added summary message:\n {}",
            self.summary_messages.last().unwrap().content
        );
    }
}
