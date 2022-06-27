FROM rust:1.61 as builder

RUN apt-get update

RUN apt-get install -y bluez bluetooth libdbus-1-dev pkg-config

# Build dependencies first (first iterative builds of app)
WORKDIR /tmp
RUN cd /tmp && USER=root cargo new --bin cli \
    && USER=root cargo new --lib gvm_lights \
    && USER=root cargo new --lib gvm_server
COPY cli/Cargo.toml cli/
COPY gvm_lights/Cargo.toml gvm_lights/
COPY gvm_server/Cargo.toml gvm_server/
RUN cargo build --release --manifest-path=cli/Cargo.toml

# Touch once after dependencies (bug?)
RUN rm cli/target/release/.fingerprint/gvm_* -r
COPY gvm_server/src gvm_server/src
COPY gvm_lights/src gvm_lights/src
COPY cli/src cli/src
RUN cargo build --release --manifest-path=cli/Cargo.toml

FROM debian:buster-slim
RUN apt-get update && apt-get install -y bluez bluetooth libdbus-1-3 && rm -rf /var/lib/apt/lists/*
COPY docker-entrypoint.sh /
ENTRYPOINT /docker-entrypoint.sh
EXPOSE 8631
COPY --from=builder /tmp/cli/target/release/gvm_cli /usr/local/bin/gvm_lights
