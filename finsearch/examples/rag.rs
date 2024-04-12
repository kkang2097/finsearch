use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::env;
use dotenv::dotenv;
pub mod openai;
pub mod brave;
use crate::openai::IO_LLM;







async fn execute() -> String {

    "Hello world!".to_string()
}



#[tokio::main]
async fn main() {
    dotenv().ok();
    //TODO: Write RAG procedure here
    let new_client = Client::new();

    let llm_args: Value = json!({
        "model": "gpt-3.5-turbo",
        "temperature": 0.1
    });
    let llm: IO_LLM = IO_LLM {
        llm_args: llm_args,
        api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| "~/".to_string())
    };

    let search_engine: BraveSearch = BraveSearch {
        api_key: env::var("BRAVE_API_KEY").unwrap_or_else(|_| "~/".to_string())
    };

    let query: String = "Where is Istanbul?".to_string();

    let brave_result = search_engine.brave_search(&new_client, &query);

    let result = llm.forward(&new_client,query.clone()).await; 
    println!("{:?}", result);

}
