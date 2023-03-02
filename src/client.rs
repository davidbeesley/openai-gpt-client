use log::{debug, info, warn};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::chat::{ChatMessage, ChatRequest, ChatResponse};
use crate::model_variants::ModelId;
use crate::text_completion::{TextCompletionRequest, TextCompletionResponse};
use crate::GptError;

#[derive(Debug, Clone, Copy)]
pub enum ClientProfile {
    Chat,
    Code,
}

impl ClientProfile {
    pub fn get_temperature(&self) -> f32 {
        match self {
            ClientProfile::Chat => 0.4,
            ClientProfile::Code => 0.7,
        }
    }

    pub fn get_top_p(&self) -> f32 {
        match self {
            ClientProfile::Chat => 0.9,
            ClientProfile::Code => 0.7,
        }
    }

    pub fn get_frequency_penalty(&self) -> f32 {
        match self {
            ClientProfile::Chat => 0.0,
            ClientProfile::Code => 0.2,
        }
    }

    pub fn get_presence_penalty(&self) -> f32 {
        match self {
            ClientProfile::Chat => 0.6,
            ClientProfile::Code => 0.0,
        }
    }

    pub fn get_stop(&self) -> Option<Stop> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Stop {
    Single(String),
    Multiple(Vec<String>),
}

pub struct OpenAiClient {
    client: Client,
    profile: ClientProfile,
}

impl OpenAiClient {
    pub fn new(api_key: &str, profile: ClientProfile) -> OpenAiClient {
        let headers = Self::build_headers(api_key);

        let client = Client::builder().default_headers(headers).build().unwrap();

        OpenAiClient { client, profile }
    }

    fn build_headers(api_key: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap(),
        );
        headers
    }

    #[allow(dead_code)]
    async fn get_request(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("https://api.openai.com/v1/{endpoint}");

        self.client.get(&url).send().await
    }

    async fn post_request(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> Result<Response, Error> {
        let url = format!("https://api.openai.com/v1/{endpoint}");

        // Send the request
        let response = self
            .client
            .post(&url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .json(&body)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn completion(
        &self,
        model: ModelId,
        prompt: &str,
        max_tokens: u16,
    ) -> Result<String, Error> {
        let request = TextCompletionRequest {
            model,
            prompt: prompt.to_owned(),
            max_tokens: Some(i32::from(max_tokens)),
            stop: self.profile.get_stop(),
            temperature: Some(self.profile.get_temperature() as f64),
            top_p: Some(self.profile.get_top_p() as f64),
            frequency_penalty: Some(self.profile.get_frequency_penalty() as f64),
            presence_penalty: Some(self.profile.get_presence_penalty() as f64),
            ..Default::default()
        };
        debug!("Request: {:?}", request);

        let response = self
            .post_request("completions", to_value(&request).unwrap())
            .await?;
        debug!("Response: {:?}", response);

        let body = response.text().await?;
        debug!("Body: {:?}", body);

        // Deserialize the response body as a TextCompletionResponse object
        let completion_response: TextCompletionResponse = serde_json::from_str(&body).unwrap();

        // Return the text generated by the API
        Ok(completion_response.choices.unwrap()[0].text.clone())
    }

    pub async fn chat(
        &self,
        model: ModelId,
        max_tokens: u16,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatMessage, GptError> {
        info!("Messages:\n {:#?}", messages);
        let request = ChatRequest {
            model,
            messages,
            max_tokens: Some(i32::from(max_tokens)),
            stop: self.profile.get_stop(),
            temperature: Some(self.profile.get_temperature() as f64),
            top_p: Some(self.profile.get_top_p() as f64),
            frequency_penalty: Some(self.profile.get_frequency_penalty() as f64),
            presence_penalty: Some(self.profile.get_presence_penalty() as f64),
            ..Default::default()
        };
        debug!("Request: {:?}", request);

        let response = self
            .post_request("chat/completions", to_value(&request).unwrap())
            .await?;
        debug!("Response: {:?}", response);

        let body = response.text().await?;
        debug!("Body: {:?}", body);

        // Deserialize the response body as a TextCompletionResponse object
        let chat_response: ChatResponse = serde_json::from_str(&body)?;

        if let Some(usage) = chat_response.usage {
            warn!(
                "Completion: {:?}, total ${}",
                usage,
                usage.total_tokens as f32 * 0.002 / 1000.0
            );
        }

        // Return the text generated by the API
        Ok(chat_response.choices[0].message.clone())
    }
}
