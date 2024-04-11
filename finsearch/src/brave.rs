use reqwest::Client;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value, from_str};
use std::env;
use dotenv::dotenv;
use urlencoding::encode;

//Brave SearchResponse structs, got this from Perplexity Pro 
#[derive(Debug, Clone, Deserialize, Serialize)]
struct SearchBundle {
    #[serde(rename = "type")]
    type_field: Option<String>,
    query: Option<Query>,
    web: Option<Web>,
    mixed: Option<Mixed>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Query {
    original: Option<String>,
    is_navigational: Option<bool>,
    is_news_breaking: Option<bool>,
    bad_results: Option<bool>,
    more_results_available: Option<bool>,
    should_fallback: Option<bool>,
    show_strict_warning: Option<bool>,
    spellcheck_off: Option<bool>,
    country: Option<String>,
    state: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    header_country: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Web {
    results: Vec<WebResult>,
    family_friendly: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct WebResult {
    #[serde(rename = "type")]
    type_field: Option<String>,
    subtype: Option<String>,
    url: Option<String>,
    title: Option<String>,
    description: Option<String>,
    language: Option<String>,
    is_source_local: Option<bool>, 
    is_source_both: Option<bool>,
    thumbnail: Option<Thumbnail>,
    age: Option<String>,
    page_age: Option<String>,
    meta_url: Option<MetaUrl>,
    profile: Option<Profile>,
    family_friendly: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Thumbnail {
    src: Option<String>,
    original: Option<String>,
    logo: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MetaUrl {
    scheme: Option<String>,
    netloc: Option<String>,
    path: Option<String>,
    hostname: Option<String>, 
    favicon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Profile {
    url: Option<String>,
    name: Option<String>,
    long_name: Option<String>,
    img: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Mixed {
    #[serde(rename = "type")]
    type_field: Option<String>,
    main: Option<Vec<MainElement>>,
    top: Option<Vec<Value>>,
    side: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MainElement {
    #[serde(rename = "type")]
    type_field: Option<String>,
    index: Option<u64>,
    all: Option<bool>,
}

//Brave Search structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BraveSearch {
    api_key: String
}

impl BraveSearch {
    async fn brave_search(&self, client: &Client, query: &str) -> Result<Vec<String>,Box<dyn std::error::Error>>{

        //TODO: Make sure this part works
            
        let res = client.get("https://api.search.brave.com/res/v1/web/search")
            .query(&[("q", query), ("result_filter", "web"), ("safesearch", "strict"), ("count", "5")])
            .header(ACCEPT, "application/json")
            .header(ACCEPT_ENCODING, "gzip, deflate")
            .header("X-Subscription-Token", &self.api_key)
            .send()
            .await?;

        let data = res.text().await?;
        let parsed = from_str::<SearchBundle>(&data);
        let results = &parsed.unwrap().web.unwrap().results;

        //TODO: Map the results into a Vec[String] and return the result  
        let result_vec: Vec<String> = results.into_iter().map(|x| x.clone().description.unwrap()).collect();

        Ok(result_vec)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let new_client: Client = reqwest::Client::new();
    
    let search_engine: BraveSearch = BraveSearch {
        api_key: env::var("BRAVE_API_KEY").unwrap_or_else(|_| "~/".to_string())
    };
    println!("{:?}", search_engine.api_key);

    let raw_string = "Where is Istanbul?";
    let result = search_engine.brave_search(&new_client, "Where is Istanbul?").await; 


    println!("{:?}", result.unwrap().get(0));
    Ok(())
}
