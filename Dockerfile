FROM debian:buster-slim

WORKDIR /app
COPY target/aarch64-unknown-linux-gnu/release/purple-telegraf-rust .
RUN chmod +x purple-telegraf-rust
CMD ["./purple-telegraf-rust"]
