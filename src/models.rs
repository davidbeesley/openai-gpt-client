use std::fmt::Display;

use crate::model_variants::ModelId;

#[derive(Debug, serde::Deserialize)]
pub struct Model {
    pub id: ModelId,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
    pub permission: Vec<ModelPermission>,
    pub root: String,
    pub parent: Option<String>,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model: {}", self.id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ModelPermission {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

pub async fn get_models() -> Result<Vec<Model>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.openai.com/v1/models")
        .send()
        .await?
        .json::<Vec<Model>>()
        .await?;
    Ok(response)
}

impl Default for ModelId {
    fn default() -> Self {
        ModelId::TextAda001
    }
}
