# docker build -t wik-dps-tp02-rust-single -f single-stage.dockerfile .
FROM rust:1.65
WORKDIR /app

RUN adduser --no-create-home --disabled-login --group --system www
RUN chown www -R /app
USER www

RUN cargo new --bin TP1
WORKDIR /app/TP1

COPY Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs
RUN rm -rf .git/ .gitignore

COPY ./src ./src
RUN rm ./target/release/deps/TP1*
RUN cargo build --release

CMD ./target/release/TP1