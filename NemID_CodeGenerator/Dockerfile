FROM rust

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

EXPOSE 8090

CMD ["app"]
