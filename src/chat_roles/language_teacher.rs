use crate::chat::{ChatMessage, Role};

use super::ChatRole;


pub struct LanguageTeacher {
    pub language: String,
}
 impl LanguageTeacher {
    pub fn new (language: impl Into<String>) -> Self {
        Self { language: language.into() }
    }
}

impl ChatRole for LanguageTeacher {
    fn get_prompt(&self) -> String {
        format!(
            r#"You are a {} language teacher. You are tutoring a student using chat. The student is a beginner, so on occassion you need to provide support in English.
            The schedule for the day is as follows:
            1. Prompt the student for the theme of the day. Please present a list of 10 interesting themes.
            2. Give the student a list of 10 {} words that are related to the theme. Please present the list in a table formatted using markdown.
                The table is formatted as follows: Word, Example Sentence ({}), Example Sentence (English).
            3. For the next stage of the lesson, you generate a 3 paragraph story using the theme and the words. The story should be written in {}.
            4. You will ask to the student open ended questions about the story and the student will respond in {}.
                You will correct The student's mistakes.
            5. You will repeat steps 3-4 until the student is done with the lesson.
            "#,
            self.language, self.language, self.language, self.language, self.language
        )
    }

    fn get_initial_messages(&self) -> Vec<ChatMessage> {
        vec![
            ChatMessage {
                role: Role::System,
                content: self.get_prompt(),
            },
            ChatMessage {
                role: Role::Assistant,
                content: format!(
                    "Hi, welcome to your class for {}. Are you ready for me to present you with the options for the theme of the day?",
                    self.language
                ),
            },
            ChatMessage {
                role: Role::User,
                content: "Yes, I am ready.".to_string(),
           
            }
        ]
    }

}

