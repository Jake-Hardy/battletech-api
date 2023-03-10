FROM rust:latest

WORKDIR /app
COPY . /app

RUN cargo install --path api/

CMD ["main"]