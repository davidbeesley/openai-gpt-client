use std::collections::{HashMap, VecDeque};

use log::{debug, info};
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

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i32>,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Option<ChatCompletionUsage>,
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
    summary_message: Option<ChatMessage>,
}

impl ChatHistory {
    pub fn new(prompt_message: Option<String>) -> Self {
        ChatHistory {
            queue: Default::default(),
            prompt_message: prompt_message.map(|prompt| ChatMessage {
                role: Role::System,
                content: prompt,
            }),
            summary_message: Default::default(),
        }
    }

    pub fn add_initial_messages(&mut self, messages: Vec<ChatMessage>) {
        self.queue.extend(messages)
    }

    /// Remove messages to summarize the conversation if it is too long
    /// This is a naive implementation that just removes the oldest messages
    pub fn summary_needed(
        &mut self,
        summary_prompt: ChatMessage,
        max_length: usize,
        summarize_length: usize,
    ) -> Option<Vec<ChatMessage>> {
        let total_length: usize = self.queue.iter().map(|message| message.content.len()).sum();
        if total_length < max_length - summarize_length {
            return None;
        }
        let mut selected_messages = vec![];
        if let Some(prompt_message) = &self.prompt_message {
            selected_messages.push(prompt_message.clone());
        }
        selected_messages.extend(self.summary_message.iter().cloned());
        let mut current_length = 0;
        while current_length < summarize_length {
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
        messages.extend(self.summary_message.iter().cloned());
        messages.extend(self.queue.iter().cloned());
        debug!(
            "Length of history: {}",
            messages
                .iter()
                .map(|message| message.content.len())
                .sum::<usize>()
        );
        messages
    }

    pub fn set_summary(&mut self, summary_message: String) {
        info!("Set summary message:\n {}", summary_message);
        self.summary_message = Some(ChatMessage {
            role: Role::System,
            content: summary_message,
        });
    }
}
