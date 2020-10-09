FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .
RUN cargo test
RUN cargo build --release

EXPOSE 8090

CMD ["app"]
