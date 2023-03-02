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
            r#"You are a {} language teacher. You are tutoring a student using chat. The student is a beginner, so on occassion you need to provide support in English. You will always correct the student's {} grammar and spelling before responding.
            You are 25 years old, female, kind, attractive, and enthusiastic. You enjoy flirting with the student but you make sure to not cross professional boundaries too far.
            The schedule for the day is as follows:
            1. Prompt the student for the theme of the day. Please present a list of 10 interesting themes and wait for the student to choose one. Choose themes that would be useful for a beginner.
            2. Give the student a list of 10 {} words that are related to the theme. Please present the list in a table formatted using markdown.
                The table is formatted as follows: Word, Example Sentence ({}), Example Sentence (English).
            3. For the next stage of the lesson, you generate a 3 paragraph story using the theme and the words. The story should be written in {}.
            4. You will ask to the student 3 reading comprehension questions about the story one at a time and the student will respond in {}.
            5. You will repeat giving the student stories and asking questions until the student is done with the lesson.
            "#,
            name, name, name, name, name, name
        )
    }
    fn get_initial_messages(&self) -> Vec<ChatMessage> {
        self.language.get_initial_messages()
    }
}

pub enum Language {
    French,
    Spanish,
    Portuguese,
    English,
}

const ENGLISH_PROMPTS: [&'static str; 4] = ["Hi, welcome to your class for English. How are you doing today?",
"I doing well. How you doing?",
"Correction: \"I am doing well. How are you doing?\"\n\nAre you ready for me to present you with the options for the theme of the day?",
"Yes, I am ready."
];

const FRENCH_PROMPTS: [&'static str; 4] = ["Salut, bienvenue à votre cours de français. Comment allez-vous aujourd'hui ?",
"Je vais bien. Et toi ?",
"Correction: \"Et vous ?\"\n\nÊtes-vous prêt(e) pour que je vous présente les options pour le thème du jour ?",
"Oui, je suis prêt(e)."
];

const SPANISH_PROMPTS: [&'static str; 4] = ["Hola, bienvenido(a) a tu clase de español. ¿Cómo estás hoy?",
"Estoy bien. ¿Y tú?",
"Corrección: \"¿Y usted?\"\n\n¿Estás listo(a) para que te presente las opciones para el tema del día?",
"Sí, estoy listo(a)."
];

const PORTUGUESE_PROMPTS: [&'static str; 4] = ["Olá, bem-vindo(a) à sua aula de português. Como você está hoje?",
"Estou bem. E você?",
"Correção: \"E o senhor/ a senhora?\"\n\nVocê está pronto(a) para que eu apresente as opções para o tema do dia?",
"Sim, estou pronto(a)."
];

impl Language {
    pub fn get_english_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "French",
            Language::Spanish => "Spanish",
            Language::Portuguese => "Portuguese",
        }
    }
    pub fn get_initial_messages(&self) -> Vec<ChatMessage> {
        let mut messages = vec![];
        for (i, prompt) in self.get_prompts().iter().enumerate() {
            let is_assistant = i % 2 == 0;
            let sender = if is_assistant {
                Role::Assistant
            } else {
                Role::User
            };
            let message = ChatMessage {
                role: sender,
                content: prompt.to_string(),
            };
            messages.push(message);
        }
        messages
    }

    pub fn get_prompts(&self) -> [&'static str; 4] {
        match self {
            Language::English => ENGLISH_PROMPTS,
            Language::French => FRENCH_PROMPTS,
            Language::Spanish => SPANISH_PROMPTS,
            Language::Portuguese => PORTUGUESE_PROMPTS,
        }
    }
}
