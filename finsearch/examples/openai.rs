use std::collections::HashMap;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct IO_LLM {
    llm_args: HashMap<String, String>,
    api_key: String
}

impl IO_LLM {
    async fn forward(&self, client: Client, query: String) -> Result<&str, Box<dyn std::error::Error>> {
        //let response = reqwest::get()
        let payload = json!({
            "field1": "Something"
        });
        let res = client.post("http://httpbin.org/post")
            .json(&payload)
            .send()
            .await
            .unwrap();
        Ok("Something")
    }
}

#[tokio::main]
async fn main() {
    let new_client: Client = reqwest::Client::new();
    
    let llm_args: HashMap<String, String> = HashMap::from([
        ("arg1".to_string(), "value".to_string())
    ]);
    
    let llm: IO_LLM = IO_LLM {
        llm_args: llm_args,
        api_key: "KEEYYYYY".to_string()
    };


}
