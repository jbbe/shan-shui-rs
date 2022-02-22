# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:1.57.0 AS build
WORKDIR /app
COPY core /app/core
COPY server /app/server
WORKDIR /app/server
RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo build --release
# # Download the target for static linking.

# # Create a dummy project and build the app's dependencies.
# # If the Cargo.toml or Cargo.lock files have not changed,
# # we can use the docker build cache and skip these (typically slow) steps.
# RUN USER=root cargo new url-shortener
# WORKDIR /usr/src/url-shortener
# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release

RUN cargo install --target x86_64-unknown-linux-musl --path .
# # Copy the source and build the application.
# COPY . .


# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/shan-shui-server .
USER 1000

EXPOSE 80
CMD ["./shan-shui-server", "80"]
# CMD ["./server/target/release/shan-shui-server"]