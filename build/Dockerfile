# Cross-compile the app for musl to create a statically-linked binary for alpine.
FROM --platform=$BUILDPLATFORM rust:latest AS rust
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
      "linux/arm/v7") echo armv7-unknown-linux-musleabihf > /rust_target.txt ;; \
      "linux/arm/v6") echo arm-unknown-linux-musleabihf > /rust_target.txt ;; \
      "linux/amd64") echo x86_64-unknown-linux-musl > /rust_target.txt ;; \
      *) exit 1 ;; \
    esac
RUN rustup target add $(cat /rust_target.txt)
RUN apt-get update && apt-get install -y binutils-arm-linux-gnueabihf g++-aarch64-linux-gnu libc6-dev-arm64-cross musl-tools musl-dev build-essential gcc-x86-64-linux-gnu
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --verbose --release --target $(cat /rust_target.txt)
# Move the binary to a location free of the target since that is not available in the next stage.
RUN cp target/$(cat /rust_target.txt)/release/normally-closed .

FROM alpine:3.16
ENV \
    # Show full backtraces for crashes.
    RUST_BACKTRACE=full
RUN apk add --no-cache \
      tini \
    && rm -rf /var/cache/* \
    && mkdir /var/cache/apk
WORKDIR /app
COPY --from=rust /app/normally-closed ./

ENTRYPOINT ["/sbin/tini", "--"]
CMD ["/app/normally-closed", "--http-port", "80", "/config/config.toml"]

EXPOSE 80
HEALTHCHECK --interval=1m --timeout=3s \
  CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1
