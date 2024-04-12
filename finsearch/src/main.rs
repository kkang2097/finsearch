use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::post,
    Router,
    serve
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer
};
use std::env;
use serde_json::{json};
use dotenv::dotenv;
pub mod openai;
pub mod brave;
use crate::openai::IO_LLM;




#[derive(Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Clone, Serialize, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatMessage>,
}


async fn chat_completions(State(state): State<AppState>, Json(request): Json<ChatCompletionRequest>) -> impl IntoResponse {
    
    //Packages the response here
    let resp_content = if let Some(_last_message) = request.messages.last() {
        if true {
            let msg = request.messages.last().unwrap().content.clone();
            //TODO: Fuse the messages into a single string using map()
            let result_vec: Vec<String> = request.messages.into_iter().map(|x| {
                                                                           let clone = x.clone();
                                                                           format!(
                                                                               "
                                                                                --------------------------------------------
                                                                                    {}: {}
                                                                                --------------------------------------------
                                                                               
                                                                                ", clone.role, clone.content
                                                                               )
                                                                            }
                                                                           ).collect(); 
            let msg_bulk = "Here is a sequence of messages: \n".to_string() + &result_vec.join("\n");
            //TODO: RAG 
            let brave_results = state.search_engine.brave_search(&state.client, &msg).await.unwrap().join("\n");
            let rag_query = format!(
                "
                    Here is the original message sequence:
                    {}

                    I used a search engine to pull relevant results. Here are some potentially useful fact snippets:
                    {}


                    Using the fact snippets and prior knowledge given above, answer the original message sequence to the best of your knowledge.
                    Output should be in STRING format and not return the original message sequence.
                ",
                &msg_bulk,
                &brave_results
                );

            let synthesized = state.llm.forward(&state.client, rag_query.to_string()).await;
            synthesized
        } else {
            //Okay there's a bug here. If you send messages such that the last message is not a
            //user message, you'll get the API to panic.
            //But we gotta get this V1 version presentable as fast as possible.
            Err("There were no messages!".into())
        }
    } else {
        Err("There were no messages!".into())
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
            content: resp_content.unwrap(),
        }],
    })
}


#[derive(Clone)]
struct AppState {
    client: Client,
    llm: openai::IO_LLM,
    search_engine: brave::BraveSearch
}


#[tokio::main]
async fn main() {

    dotenv().ok();
    
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();


    let app_state = AppState {
    client: Client::new(),
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

    let cors = CorsLayer::new()
        .allow_origin(Any);

    let app = Router::new()
        .with_state(app_state.clone())
        .route("/chat/completions", post(chat_completions))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);
    let port = std::env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    serve(listener, app).await.unwrap();
}
