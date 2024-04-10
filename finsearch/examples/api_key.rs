use std::env;


fn main() {
    let api_key = env::var("BRAVE_API_KEY").unwrap_or_else(|_| "~/".to_string());
    println!("{:?}", api_key);
}
