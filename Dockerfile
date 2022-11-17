FROM rust:latest AS builder

WORKDIR /usr/src/
RUN cargo new clicky

WORKDIR /usr/src/clicky
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11	

COPY --from=builder /usr/local/cargo/bin/clicky .
USER 1000
ENTRYPOINT ["./clicky"]