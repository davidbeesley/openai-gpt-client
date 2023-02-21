use log::info;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error, Response};
use serde_json::{json, to_string, to_value};

use crate::conversations::{TextCompletionRequest, TextCompletionResponse};
use crate::model_variants::ModelId;
use crate::models::Model;

pub struct OpenAiClient {
    client: Client,
}

impl OpenAiClient {
    pub fn new(api_key: &str) -> OpenAiClient {
        let headers = Self::build_headers(&api_key);

        let client = Client::builder().default_headers(headers).build().unwrap();

        OpenAiClient { client }
    }

    async fn get_request(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("https://api.openai.com/v1/{}", endpoint);

        self.client.get(&url).send().await
    }

    async fn post_request(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> Result<Response, Error> {
        let url = format!("https://api.openai.com/v1/{}", endpoint);

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

    fn build_headers(api_key: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers
    }

    pub async fn get_models(&self) -> Result<Vec<Model>, Error> {
        let response = self.get_request("models").await?;

        let body = response.text().await?;
        let mut model_list: ModelList = serde_json::from_str(&body).unwrap();
        model_list.data.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(model_list.data)
    }

    pub async fn send_text(
        &self,
        model: ModelId,
        prompt: &str,
        temperature: f64,
        max_tokens: i32,
    ) -> Result<TextCompletionResponse, Error> {
        // Create the request body as a TextCompletionRequest object
        let request_body = TextCompletionRequest {
            model,
            prompt: prompt.to_owned(),
            temperature: Some(temperature),
            max_tokens: Some(max_tokens),
            ..Default::default()
        };

        // Send the request using the post_request function
        let response = self
            .post_request("completions", to_value(&request_body).unwrap())
            .await?;
        info!("Response: {:?}", response);

        // Deserialize the response body as a TextCompletionResponse object
        let completion_response: TextCompletionResponse = response.json().await?;

        // Return the TextCompletionResponse object
        Ok(completion_response)
    }
    //
    //
    // pub async fn send_text(
    //     &self,
    //     model: ModelId,
    //     prompt: &str,
    //     temperature: i32,
    //     max_tokens: i32,
    // ) -> Result<String, Error> {
    //     // Create the request body as a JSON object
    //     let request_body = json!({
    //         "model": model,
    //         "prompt": prompt,
    //         "temperature": temperature,
    //         "max_tokens": max_tokens,
    //     });

    //     let request_body_2 = to_value(&TextCompletionRequest {
    //         model,
    //         prompt: prompt.to_owned(),
    //         temperature: Some(0.0),
    //         max_tokens: Some(max_tokens),
    //         ..Default::default()
    //     })
    //     .unwrap();

    //     println!("request_body: {:#?}", request_body);
    //     println!("request_body_2: {:#?}", request_body_2);
    //     // Send the request
    //     // let response = self
    //     //     .client
    //     //     .post("https://api.openai.com/v1/completions")
    //     //     .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
    //     //     .json(&request_body)
    //     //     .send()
    //     //     .await?;
    //     let response = self.post_request("completions", request_body_2).await?;

    //     // Get the response body
    //     let body = response.text().await?;

    //     // Return the response body as a String
    //     Ok(body)
    // }
}

#[derive(Debug, serde::Deserialize)]
struct ModelList {
    data: Vec<Model>,
}
