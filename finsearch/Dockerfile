FROM rust:1.72 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .env ./
RUN cargo build --release
EXPOSE 7878
CMD ["./target/release/finsearch"]