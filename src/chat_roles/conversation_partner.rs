use crate::chat::ChatMessage;

use super::{language_teacher::Language, ChatRole};

pub struct ConversationPartner {
    pub language: Language,
}
impl ConversationPartner {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

impl ChatRole for ConversationPartner {
    fn get_prompt(&self) -> String {
        let name = self.language.get_english_name();
        format!(
            r#"Your name is {}.
You speak {name}. You are 25, female, engaging, intelligent, and friendly.
You are a conversation partner helping a user who is learning {name}.

Do not break character."#,
            self.language.get_name(),
        )
    }
    fn get_initial_messages(&self) -> Vec<ChatMessage> {
        vec![ChatMessage {
            role: crate::chat::Role::System,
            content: "Begin by introducing yourself".to_owned(),
        }]
    }
}
