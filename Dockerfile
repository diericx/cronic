FROM --platform=$BUILDPLATFORM rust:latest AS rust

# Capture target platform in a file for later use
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
      "linux/amd64") echo x86_64-unknown-linux-musl> /rust_target.txt ;; \
      *) exit 1 ;; \
    esac

RUN rustup target add $(cat /rust_target.txt)
RUN apt-get update && apt-get install -y musl-tools

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo test
RUN cargo build --verbose --release --target $(cat /rust_target.txt)

# Move the binary to a location free of the target since that is not available in the next stage.
RUN cp target/$(cat /rust_target.txt)/release/cronic .

FROM alpine:3.16
ENV \
    # Show full backtraces for crashes.
    RUST_BACKTRACE=full
WORKDIR /app
COPY --from=rust /app/cronic  ./
COPY Rocket.toml ./
COPY templates ./templates

CMD ["/app/cronic"]

EXPOSE 80
