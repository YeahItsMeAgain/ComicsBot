FROM rust:1.89 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release


# Runtime stage
FROM ubuntu:22.04
WORKDIR /app

# Install required libraries
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/comics-bot .
CMD ["./comics-bot"]
