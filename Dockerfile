FROM rust:latest

WORKDIR /app
COPY . /app

RUN cd api
RUN cargo install --path .

CMD ["main"]