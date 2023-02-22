//! use std::env;
//!
//! use chatgpt::{client::OpenAiClient, model_variants::ModelId};
//!
//! use reqwest::Error;
//!
//! # Examples
//! ```
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     env_logger::init();
//!     // Set the API key and request parameters
//!     let api_key = env::var("OPENAI_API_KEY").unwrap();
//!     let client = OpenAiClient::new(&api_key);
//!
//!     let model = ModelId::TextCurie001;
//!     let prompt = r#"
//! The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.
//!
//! Human: Hello, who are you?
//! AI: I am an AI created by OpenAI. How can I help you today?
//! Human: You are a college professor. You are writing a textbook on Computer Science. Please give the table on contents for this book. Please include 10 chapters
//! AI: "#;
//!     let max_tokens = 1000;
//!
//!     let res = client.send_text(model, prompt, max_tokens).await?;
//!     println!("Response: {:#?}", res);
//!     println!("Response:\n{}", res);
//!     let res = client.send_text(model, prompt, max_tokens).await?;
//!     println!("Response: {:#?}", res);
//!     println!("Response:\n{}", res);
//!
//!     Ok(())
//! }
//! ```
//!
pub mod client;
pub mod conversations;
pub mod model_variants;
pub mod models;
