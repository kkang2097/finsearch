use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::env;
use json_value_merge::Merge;
use urlencoding::encode;


//Brave Search Response structs
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BraveSearchResults {
    results: Vec<BraveSearchResponse>
}

pub struct BraveSearch {
    api_key: String
}

impl BraveSearch {
    async fn brave_search(&self, client: &Client, query: &str) -> Result<BraveSearchResponse,Box<dyn std::error::Error>>{

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip".parse().unwrap());
        headers.insert("X-Subscription-Token", self.api_key.parse().unwrap());

        println!("{:?}", headers);

        //TODO: Make sure this part works
        let res = client.post("https://api.search.brave.com/res/v1/web/search?q=".to_string() + &encode(query))
            .headers(headers)
            .send()
            .await?;
        
        //TODO: Parse the struct with serde_json and return the result
        if res.status().is_success() {
           let data: BraveSearchResponse = res.json().await?;  
            return Ok(data);
        }
        Err("Error".into())
    }
}

#[tokio::main]
async fn main() {
    let new_client: Client = reqwest::Client::new();
    
    let search_engine: BraveSearch = BraveSearch {
        api_key: env::var("BRAVE_API_KEY").unwrap_or_else(|_| "~/".to_string())
    };
    println!("{:?}", search_engine.api_key);

    let raw_string = "Where is Istanbul?";
    let result = search_engine.brave_search(&new_client, "Where is Istanbul?").await; 
    println!("{:?}", result);
    ()
}
