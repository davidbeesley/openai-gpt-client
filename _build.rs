use convert_case::{Case, Casing};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
const PREFIX: &str = r#"
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ModelId {
"#;

const SUFFIX: &str = r#"
}"#;

#[derive(Debug, Deserialize)]
struct Model {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ModelList {
    data: Vec<Model>,
}

fn main() {
    // Read the API key from an environment variable
    let api_key = env::var("OPENAI_API_KEY").unwrap();

    // Set the headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Set the headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create the client and send the request
    let client = Client::new();
    let response = client
        .get("https://api.openai.com/v1/models")
        .headers(headers)
        .send()
        .unwrap();

    // Deserialize the response body
    let body = response.text().unwrap();
    let model_list: ModelList = serde_json::from_str(&body).unwrap();

    let mut inputs = model_list
        .data
        .into_iter()
        .map(|model| model.id)
        .collect::<Vec<String>>();
    inputs.sort();

    let code = generate_code(&inputs);
    let mut file = File::create("src/model_variants.rs").expect("Failed to create file");
    write!(file, "{}", PREFIX).expect("Failed to write to file");
    write!(file, "{}", code).expect("Failed to write to file");
    write!(file, "{}", SUFFIX).expect("Failed to write to file");
}

fn generate_code(inputs: &[String]) -> String {
    inputs
        .iter()
        .enumerate()
        .map(|(i, input)| {
            println!("{}", input);
            let id = input.to_owned();
            let id = id.replace(".", "PERIOD");
            let id = id.replace(":", "COLON");
            format!(
                "#[serde(rename = \"{}\")]\n{}{}{}",
                input,
                id.to_case(Case::Pascal),
                "",
                if i < inputs.len() - 1 { "," } else { "" }
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
