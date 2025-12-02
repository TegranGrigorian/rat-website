# Make docker file for rat website, axum rust
FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["cargo", "run", "--release"]
