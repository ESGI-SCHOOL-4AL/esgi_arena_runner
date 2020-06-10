FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

EXPOSE 8088

CMD ["./target/release/esgi_arena_runner"]
