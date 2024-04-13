# finsearch
Ultra-slow-but-accurate RAG + Search Engine API for Financial QA

### Hard parts
- reverse engineering Brave Search's endpoints
- figuring out Rust things on the fly (mostly common sense + Perplexity)
- using derived traits in Rust
- finding a right balance between copy pasta and regular coding using Perplexity

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
- ~~FreshEval~~
FreshEval actually relies on function calling, so it'll need some tweaks to work on the vanilla /chat/completions endpoint
### V4
- Docker multi-stage build (20x smaller Docker image)
- Decouple code, multi-threaded server

###V5
- automatic parallelism of RAG chains, which involves optimizing the computation graph

## Some observations about AI-assisted coding
I've been pretty opposed to using ChatGPT for coding. But this past week, I have to say I've completely changed my stance on AI-assisted coding.

On Sunday, I saw someone on our hackathon team go 'TAB TAB TAB TAB a : b ? TAB TAB TAB TAB TAB' on some frontend code to write something that would have taken several minutes. It took him 15 seconds.

This past two days, I wrote an OpenAI-compatible API in Rust. I've barely coded in Rust other than the classic Rustlings exercises. It went like
- write some logic
- use Perplexity to answer some questions about docs
- copy pasta for the first draft
- debug debug debug, mix of programming knowledge + previous experience
- write some more logic
- compile, rinse and repeat

Intuition and programming skills matter more than ever actually, since you wouldn't know the difference between good and bad code. GenAI based search still hallucinates. AI assistance just lets you iterate faster.

### Good ways to use GenAI for coding assistance
- getting a first draft out
- asking simple straightforward questions like "How do I add tracing to an Axum API in Rust?"
- quick summaries about a variety of topics
- using Perplexity Pro + Opus and asking follow-up questions to get the right output

GenAI helps for sure. It's ~just accurate enough to meaningfully improve developer productivity IMO.
The level of skill in querying Perplexity matters too I think. Follow-up questions are really important.

### Bad ways to use GenAI for coding assistance
- blind copy pasta, a lot of code output by even Opus + Perplexity doesn't even compile
- expecting GenAI to do all the work. I tried getting Perplexity to build a multi-stage Dockerfile for several hours and it failed so hard. So I scrapped everything and rewrote the Dockerfile myself using my intuition in 2 minutes.
The Dockerfile isn't optimal at all, but it works and is simple to reason about.

