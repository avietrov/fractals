FROM rust:1.66 as builder

WORKDIR /newton-factal
COPY ./src ./src
COPY ./benches ./benches
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/newton_factal"]
