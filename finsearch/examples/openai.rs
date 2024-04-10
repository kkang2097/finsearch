use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::env;
use json_value_merge::Merge;


//Internal structs
#[derive(Clone, Serialize, Deserialize)]
struct IO_LLM {
    llm_args: Value,
    api_key: String
}

#[derive(Clone, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String
}

//OpenAI Response Structs
#[derive(Debug, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Clone, Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Clone, Debug, Deserialize)]
struct Message {
    content: String,
}

//Brave Search Response structs
#[derive(Debug, Serialize, Deserialize)]
struct BraveSearchResponse {
    quality_score: f64,
    rank: u32,
    title: String,
    url: String,
    snippet: String,
    thumbnail: Option<String>,
    source: String,
    published_at: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct BraveSearchResults {
    results: Vec<BraveSearchResponse>
}

pub fn package_openai_response(str_response: String) -> ApiResponse {
    //TODO: Package a string response into an ApiResponse
    let response: ApiResponse = ApiResponse {
        choices: vec![Choice {
            message: Message {
                content: str_response
            }
        }]
    };
    response
}

pub fn brave_search(client: &Client, query: &str) {
}


impl IO_LLM {
    async fn forward(&self, client: &Client, query: String) -> Result<String, Box<dyn std::error::Error>> {
        //let response = reqwest::get()
        
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", ["Bearer ", &self.api_key].concat().parse().unwrap());

        let messages: Vec<OpenAIMessage> = vec![OpenAIMessage {role: String::from("user"), content: query}];
        let mut prompt = json!({
            "messages": messages
        });
        prompt.merge(&self.llm_args);
        
        println!("{:?}", prompt);

        let res = client.post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&prompt)
            .send()
            .await?;

        if res.status().is_success() {
            let data: ApiResponse = res.json().await?;
            let msg_clone: Choice = data.choices.get(0).clone().unwrap();
            let msg_string: String = msg_clone.message.content;
            return Ok(msg_clone);
        }
        Err("Error".into())
    }
}

#[tokio::main]
async fn main() {
    let new_client: Client = reqwest::Client::new();
    
    let llm_args: Value = json!({
        "model": "gpt-3.5-turbo",
        "temperature": 0.1
    });
    
    let llm: IO_LLM = IO_LLM {
        llm_args: llm_args,
        api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| "~/".to_string())
    };

    let result = llm.forward(&new_client,String::from("Where is Istanbul?")).await; 
    println!("{:?}", result);
    ()
}
