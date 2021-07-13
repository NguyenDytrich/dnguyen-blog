FROM rust:1.53-buster as builder

WORKDIR /usr/src
RUN USER=root cargo new --bin dnguyen-blog
WORKDIR ./dnguyen-blog
COPY ./Cargo.toml ./Cargo.toml
RUN cargo install --path .
RUN rm src/*.rs

ADD . ./
RUN cargo build

FROM debian:buster-slim
ARG APP=/usr/src/app
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/dnguyen-blog/target/debug/server ${APP}/dnguyen-blog
WORKDIR ${APP}
EXPOSE 8000

CMD ["./dnguyen-blog"]
