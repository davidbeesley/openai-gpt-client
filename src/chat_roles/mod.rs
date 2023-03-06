pub mod conversation_partner;
pub mod language_teacher;
use crate::chat::{ChatMessage, Role};

pub trait ChatRole {
    fn get_prompt(&self) -> String;
    fn get_initial_messages(&self) -> Vec<ChatMessage>;
    fn get_summary_prompt(&self) -> ChatMessage {
        ChatMessage {
            role: Role::System,
            content: " Please summarize the previous conversation in a paragraph.".to_owned(),
        }
    }
}
