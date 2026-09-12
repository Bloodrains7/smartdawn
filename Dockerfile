FROM rust:1.93.1-bookworm AS build
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked
COPY hero-wasm/Cargo.toml hero-wasm/Cargo.lock ./hero-wasm/
COPY hero-wasm/src ./hero-wasm/src
RUN cargo build --manifest-path hero-wasm/Cargo.toml --target wasm32-unknown-unknown --release --locked
COPY assets ./assets
RUN cp hero-wasm/target/wasm32-unknown-unknown/release/hero_wasm.wasm assets/hero.wasm

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN useradd --system --uid 10001 --no-create-home app
COPY --from=build /app/target/release/smart-dawn-web /app/smart-dawn-web
COPY --from=build /app/assets /app/assets
ENV PORT=3000 RUST_LOG=info,tower_http=info
USER 10001
EXPOSE 3000
CMD ["/app/smart-dawn-web"]
