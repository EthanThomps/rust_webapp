FROM rust:latest
# FrontendImage
RUN cargo install bonnie 
RUN cargo install dioxus-cli
RUN cargo install -f wasm-bindgen-cli --version 0.2.86
RUN rustup target add wasm32-unknown-unknown
RUN cargo add dioxus 
RUN cargo add dioxus-web
RUN cargo add dioxus-ssr
RUN cargo add dioxus-free-icons -F ionicons
RUN cargo add dioxus_router
RUN cargo update -p wasm-bindgen --precise 0.2.87
RUN cargo report future-incompatibilities 
RUN cargo test
RUN cargo build

# todo! copy, workdir

CMD ["dioxus", "serve"]