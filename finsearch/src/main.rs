use axum::{
    extract::State,
    http::{StatusCode, Request, Response},
    response::{IntoResponse, Json},
    routing::post,
    middleware::{Next, from_fn},
    Router,
    serve
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{
    trace::TraceLayer
};
use std::env;
use serde_json::{json, Value};
use dotenv::dotenv;

pub mod openai;
pub mod brave;
use crate::openai::IO_LLM;
use crate::brave::BraveSearch;




#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatMessage>,
}

async fn chat_completions(State(state): State<AppState>, Json(request): Json<ChatCompletionRequest>) -> impl IntoResponse {
    let resp_content = if let Some(first_message) = request.messages.first() {
        if first_message.role == "user" {
            format!("As a mock AI Assistant, I can only echo your last message: {}", request.messages.last().unwrap().content)
        } else {
            "There were no messages!".to_string()
        }
    } else {
        "There were no messages!".to_string()
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Json(ChatCompletionResponse {
        id: "1337".to_string(),
        object: "chat.completion".to_string(),
        created: now,
        model: request.model,
        choices: vec![ChatMessage {
            role: "assistant".to_string(),
            content: resp_content,
        }],
    })
}


#[derive(Clone)]
struct AppState {
    http_client: Client,
    llm: openai::IO_LLM,
    search_engine: brave::BraveSearch
}


#[tokio::main]
async fn main() {

    dotenv().ok();
    
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();


    let llm_args: Value = json!({
        "model": "gpt-3.5-turbo",
        "temperature": 0.1
    });
    
    let llm: IO_LLM = IO_LLM {
        llm_args: llm_args,
        api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| "~/".to_string())
    };

    let app_state = AppState {
        http_client: Client::new(),
        llm: IO_LLM {
            api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| "~/".to_string()),
            llm_args: json!({
                "model": "gpt-3.5-turbo",
                "temperature": 0.1
            })
        },
        search_engine: brave::BraveSearch {
            api_key: env::var("BRAVE_API_KEY").unwrap_or_else(|_| "~/".to_string()),
        }
    };


    let app = Router::new()
        .with_state(app_state)
        .route("/chat/completions", post(chat_completions))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
        serve(listener, app)
        .await
        .unwrap();
}
