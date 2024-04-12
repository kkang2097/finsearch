# finsearch
Ultra-fast RAG + Search Engine API for Financial QA



### Note: Parsing API calls using Brave is actually really complicated
For example, look at this (result)[https://github.com/kayvane1/brave-api/tree/staging/src/brave/types]



#### Observation: We're primarily focused on just the web results and news, so we'll only include those in the Rust struct for WebResponse

- Decomposed HTTP responses from Brave Search API into Rust structs
- Found some code online to mimic the OpenAI Chat Completion endpoint (you can also decompose the endpoint the same way I did for Brave)
- Set up Axum things like tracing, app state, etc
- added a little bit of RAG (Brave Search + OpenAI synthesizer)



