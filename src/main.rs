use std::{env, io};

use chatgpt::{
    chat::{ChatHistory, ChatMessage, Role},
    client::{ClientProfile, OpenAiClient},
    model_variants::ModelId,
    GptError,
};

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
    let max_tokens = 1000;

    let res = client.completion(model, prompt, max_tokens).await?;
    println!("Response:\n{}", res);

    Ok(())
}

async fn chat_with_ai() -> Result<(), GptError> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = OpenAiClient::new(&api_key, ClientProfile::Chat);
    let model = ModelId::Gpt3Period5Turbo;
    let max_tokens = 50;
    let mut chat_history = ChatHistory::new();

    loop {
        // Get input from user
        let mut input = String::new();
        println!("You: ");
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // If user wants to exit, break the loop
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Create chat message from user input and add it to chat history
        let user_message = ChatMessage {
            role: Role::User,
            content: input.to_string(),
        };
        chat_history.add_message(user_message);

        // Chat with AI
        let ai_message = client
            .chat(model, max_tokens, chat_history.get_history().to_vec())
            .await?;

        chat_history.add_message(ai_message.clone());

        // Print AI response
        println!("AI: {}", ai_message.content);
    }

    Ok(())
}
