FROM rust:1.67 as builder

WORKDIR /usr/src/einwurf

# Create project to build and cache dependencies.
RUN cargo init --bin
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release && \
    rm ./src/main.rs && \
    rm ./target/release/deps/einwurf*

# Add and compile actual source code.
COPY ./src ./src
RUN cargo build --release

FROM rust:1.67-slim-bullseye
COPY --from=builder /usr/src/einwurf/target/release/einwurf .
CMD ["./einwurf", "--config", "config.toml"]
