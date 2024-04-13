# finsearch
Ultra-fast RAG + Search Engine API for Financial QA

### Hard parts
- reverse engineering Brave Search's endpoints
- figuring out Rust things on the fly (mostly common sense + Perplexity)
- using derived traits in Rust


### Approach
- Make each component in isolation and bring it together later
- Integration is the easy part
- Keep things simple & functional where possible

### Note: Parsing API calls using Brave is actually really complicated
For example, look at this (result)[https://github.com/kayvane1/brave-api/tree/staging/src/brave/types]

#### V1 (done)
- Decomposed HTTP responses from Brave Search API into Rust structs (Thanks Perplexity/Opus)
- Found some code online to mimic the OpenAI Chat Completion endpoint, but had to tweak it quite a bit (There's almost no reference Rust code online)
- Set up Axum things like tracing, app state, etc
- added a little bit of RAG (Brave Search + OpenAI synthesizer)

### V2 (done)
- Dockerize Image, Push to GCP

### V3
- FreshEval

### V4
- Docker multi-stage build (20x smaller Docker image)
- Decouple code, multi-threaded server


