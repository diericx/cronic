FROM rust:latest as builder

# create a new empty shell project
RUN USER=root cargo new --bin cronic 
WORKDIR /cronic

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/cronic*
RUN cargo build --release

FROM rust:latest

COPY --from=builder /cronic/target/release/cronic .
COPY ./templates ./templates 
COPY ./Rocket.toml ./Rocket.toml

CMD ["./cronic"]
