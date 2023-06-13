FROM rust:latest
# BackendImage
RUN cargo install bonnie 
RUN cargo add axum
RUN cargo add tokio -F full
RUN cargo add serde -F derive
RUN cargo add hyper -F full 
RUN cargo add tower

RUN cargo report future-incompatibilities 
RUN cargo test
RUN cargo build

# todo! copy, workdir

CMD ["cargo", "run"]