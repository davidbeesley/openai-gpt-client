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

const MIDDLE: &str = r#"
}

impl core::fmt::Display for ModelId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
"#;

const SUFFIX: &str = r#"
        };
        write!(f, "{}", s)
    }
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
        .iter()
        .map(|model| model.id.clone())
        .collect::<Vec<String>>();
    inputs.sort();

    let enum_code = generate_enum(&inputs);
    let display_code = generate_display(&inputs);
    let mut file = File::create("src/model_variants.rs").expect("Failed to create file");
    write!(file, "{}", PREFIX).expect("Failed to write to file");
    write!(file, "{}", enum_code).expect("Failed to write to file");
    write!(file, "{}", MIDDLE).expect("Failed to write to file");
    write!(file, "{}", display_code).expect("Failed to write to file");
    write!(file, "{}", SUFFIX).expect("Failed to write to file");
}

fn generate_enum(inputs: &[String]) -> String {
    inputs
        .iter()
        .enumerate()
        .map(|(_, input)| {
            println!("{}", input);
            let id = input.to_owned();
            let id = id.replace(".", "PERIOD");
            let id = id.replace(":", "COLON");
            format!("{},\n", id.to_case(Case::Pascal),)
        })
        .collect::<Vec<String>>()
        .join("")
}

fn generate_display(inputs: &[String]) -> String {
    inputs
        .iter()
        .enumerate()
        .map(|(_, input)| {
            println!("{}", input);
            let id = input.to_owned();
            let id = id.replace(".", "PERIOD");
            let id = id.replace(":", "COLON");
            format!("ModelId::{} => \"{input}\",\n", id.to_case(Case::Pascal))
        })
        .collect::<Vec<String>>()
        .join("")
}
