use axum::{
    http::{StatusCode, Request, Response},
    response::{IntoResponse, Json},
    routing::post,
    middleware::{Next, from_fn},
    Router,
    serve
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{
    trace::TraceLayer
};


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

async fn chat_completions(Json(request): Json<ChatCompletionRequest>) -> impl IntoResponse {
    let resp_content = if let Some(first_message) = request.messages.first() {
        if first_message.role == "user" {
            format!("As a mock AI Assistant, I can only echo your last message: {}", request.messages.last().unwrap().content)
        } else {
            "As a mock AI Assistant, I can only echo your last message, but there were no messages!".to_string()
        }
    } else {
        "As a mock AI Assistant, I can only echo your last message, but there were no messages!".to_string()
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let app = Router::new().route("/chat/completions", post(chat_completions))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
        serve(listener, app)
        .await
        .unwrap();
}
