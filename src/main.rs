use std::env;

use chatgpt::{client::OpenAiClient, model_variants::ModelId};

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    // Set the API key and request parameters
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = OpenAiClient::new(&api_key);

    let model = ModelId::TextAda001;
    let prompt = "You are a college professor. You are writing a textbook on Computer Science. Please give the table on contents for this book. Please include 10 chapters";
    let temperature = 0.0;
    let max_tokens = 1000;

    let res = client
        .send_text(model, prompt, temperature, max_tokens)
        .await?;
    println!("Response: {:#?}", res);

    Ok(())
}
