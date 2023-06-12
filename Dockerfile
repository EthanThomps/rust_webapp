FROM rust:latest

# copies the current dir to another dir in the container
COPY . ./usr/app

# uses this dir
WORKDIR /usr/app

# installs into the workdir
RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown
RUN cargo add dioxus 
RUN cargo add dioxus-web
RUN cargo add dioxus-ssr
RUN cargo add tokio --features full
RUN cargo add axum
RUN cargo add dioxus-free-icons
RUN npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch

CMD ["dioxus", "serve"]