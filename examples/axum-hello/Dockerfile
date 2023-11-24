FROM rust:1.74-bookworm as builder

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-helloworld /usr/local/bin/rust-helloworld
ENV tz = Asia/Bangkok
EXPOSE 80

ENTRYPOINT ["rust-helloworld"]