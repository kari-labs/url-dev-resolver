FROM rust:latest

WORKDIR /usr/src/url-dev-resolver

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broked\")}" > src/main.rs

RUN cargo build

RUN rm -f target/debug/deps/url-dev-resolver*
RUN rm -rf src/

COPY . .

RUN cargo install --debug --path .

CMD ["/usr/local/cargo/bin/url-dev-resolver"]