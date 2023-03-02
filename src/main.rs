use std::{
    env,
    io::{self, Read, Write},
};

use chatgpt::{
    chat::{ChatHistory, ChatMessage, Role},
    chat_roles::{
        language_teacher::{Language::French, LanguageTeacher},
        ChatRole,
    },
    client::{ClientProfile, OpenAiClient},
    model_variants::ModelId,
    GptError,
};
use log::info;

#[tokio::main]
async fn main() -> Result<(), GptError> {
    env_logger::init();
    // Set the API key and request parameters
    // text_completion().await?;
    chat_with_ai().await
}

#[allow(dead_code)]
async fn text_completion() -> Result<(), GptError> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = OpenAiClient::new(&api_key, ClientProfile::Chat);
    let model = ModelId::TextAda001;
    let prompt = r#"
The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.

Human: Hello, who are you?
AI: I am an AI created by OpenAI. How can I help you today?
Human: You are a college professor. You are writing a textbook on Computer Science. Please give the table on contents for this book. Please include 10 chapters
AI: "#;
    let max_tokens = 200;

    let res = client.completion(model, prompt, max_tokens).await?;
    println!("Response:\n{}", res);

    Ok(())
}

async fn chat_with_ai() -> Result<(), GptError> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = OpenAiClient::new(&api_key, ClientProfile::Chat);
    let model = ModelId::Gpt3Period5Turbo;
    let max_tokens = 500;
    let chat_role = LanguageTeacher::new(French);
    let mut chat_history = ChatHistory::new(Some(chat_role.get_prompt()));
    chat_history.add_initial_messages(chat_role.get_initial_messages());
    print!("AI: ");
    loop {
        // Chat with AI
        let ai_message = client
            .chat(model, max_tokens, chat_history.get_history().to_vec())
            .await?;

        chat_history.add_message(ai_message.clone());

        // Print AI response
        println!("{}", ai_message.content);

        // Get input from user
        let mut input = String::new();
        println!("You: ");
        // flush stdout
        // io::stdin().read_to_string(&mut input)?;
        io::stdin().read_line(&mut input)?;

        // If user wants to exit, break the loop
        if input.is_empty() || input.eq_ignore_ascii_case("exit\n") {
            break;
        }
        print!("AI: ");
        io::stdout().flush().unwrap();

        // Check if the chat history requires a summary
        if let Some(summary_prompt) = chat_history.summary_needed(chat_role.get_summary_prompt()) {
            info!("Summary needed:\n {:#?}", summary_prompt);
            let summary_message = client.chat(model, max_tokens, summary_prompt).await?;
            chat_history.add_summary(summary_message.content);
        }

        // Create chat message from user input and add it to chat history
        let user_message = ChatMessage {
            role: Role::User,
            content: input.trim().to_string(),
        };
        chat_history.add_message(user_message);
    }

    Ok(())
}
