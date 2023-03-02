use crate::chat::{ChatMessage, Role};

use super::ChatRole;

pub struct LanguageTeacher {
    pub language: Language,
}
impl LanguageTeacher {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

impl ChatRole for LanguageTeacher {
    fn get_prompt(&self) -> String {
        let name = self.language.get_english_name();
        format!(
            r#"You are a {name} language teacher named {}. You are tutoring a student via instant messaging. The student is a beginner.
You are 25 years old, female, kind, and enthusiastic. Since you are using text messages, you will not be able to hear the student's pronunciation.

The schedule for each lesson is as follows:
1. Prompt the student for the theme of the day. Please present a list of 10 interesting themes and wait for the student to choose one. Choose themes that would be useful for a beginner.
2. Give the student a list of 10 {name} words that are related to the theme. Please present the list in a table formatted using markdown.
    The table is formatted as follows: Word, Translation,
3. For the next stage of the lesson, you generate a 3 paragraph story using the theme and the words. The story should be written in {name}.
4. You will ask the student a reading comprehension question about the story.
5. Choose another theme and repeat.

You will only speak english when providing a translation. Otherwise, you will speak in {name}. Please keep the stories culturally relevent to countries that speak {name}.

You always finish by asking the user a question to move the lesson forward"#,
            self.language.get_name(),
        )
    }
    fn get_initial_messages(&self) -> Vec<ChatMessage> {
        self.language.get_initial_messages()
    }
}

pub enum Language {
    English,
    French,
    Spanish,
    Portuguese,
}

impl Language {
    pub fn get_english_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "French",
            Language::Spanish => "Spanish",
            Language::Portuguese => "Portuguese",
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Language::English => "Susan",
            Language::French => "Marie",
            Language::Spanish => "Sofia",
            Language::Portuguese => "Maria",
        }
    }
    pub fn get_initial_messages(&self) -> Vec<ChatMessage> {
        let prompt = match self {
            Language::English => &ENGLISH_PROMPTS,
            Language::French => &FRENCH_PROMPTS,
            Language::Spanish => &SPANISH_PROMPTS,
            Language::Portuguese => &PORTUGUESE_PROMPTS,
        };
        prompt
            .iter()
            .map(|(role, content)| ChatMessage {
                role: *role,
                content: content.to_string(),
            })
            .collect()
    }
}

pub struct ExamplePrompt {
    assistant: &'static str,
    user: &'static str,
    corrected: &'static str,
}

impl From<&ExamplePrompt> for Vec<ChatMessage> {
    fn from(prompt: &ExamplePrompt) -> Self {
        vec![
            ChatMessage {
                role: Role::Assistant,
                content: prompt.assistant.to_owned(),
            },
            ChatMessage {
                role: Role::User,
                content: prompt.user.to_owned(),
            },
            ChatMessage {
                role: Role::System,
                content: "Please repeat what the user said with corrections.".to_owned(),
            },
            ChatMessage {
                role: Role::Assistant,
                content: prompt.corrected.to_owned(),
            },
        ]
    }
}

const ENGLISH_PROMPTS: [(Role, &'static str); 4] = [
    (Role::Assistant, "Good morning, how are you doing today?"),
    (Role::User, "You am doing good"),
    (
        Role::Assistant,
        "That's great to hear. Are you ready for me to suggest 4 themes for today?",
    ),
    (Role::User, "Yes, I ready"),
];

const FRENCH_PROMPTS: [(Role, &'static str); 4] = [
    (Role::Assistant, "Bonjour, comment allez-vous aujourd'hui ?"),
    (Role::User, "Je ves ben."),
    (
        Role::Assistant,
        "C'est formidable de l'entendre. Êtes-vous prêt pour que je suggère 4 thèmes pour aujourd'hui ?",
    ),
    (Role::User, "Oui, suis prêt."),
];

const SPANISH_PROMPTS: [(Role, &'static str); 4] = [
    (Role::Assistant, "Buenos días, ¿cómo estás hoy?"),
    (Role::User, "Estoy bueno."),
    (
        Role::Assistant,
        "Es genial escuchar eso. ¿Estás listo para que te sugiera 4 temas para hoy?",
    ),
    (Role::User, "Si, estoy listo."),
];

const PORTUGUESE_PROMPTS: [(Role, &'static str); 4] = [
    (Role::Assistant, "Bom dia, como você está hoje?"),
    (Role::User, "Estoy bem."),
    (
        Role::Assistant,
        "É ótimo ouvir isso. Você está pronto para eu sugerir 4 temas para hoje?",
    ),
    (Role::User, "Sim, estou listo."),
];
