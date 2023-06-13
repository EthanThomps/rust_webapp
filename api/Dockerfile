FROM rust:latest
# configure for api, maybe copy api into the /usr/app place

# installs into the workdir
# todo: only allow server scripts eg:: bonnie db, etc
RUN cargo install bonnie 
RUN cargo add tokio --features full
RUN cargo add axum
RUN cargo add serde -F derive

# testing
RUN cargo report future-incompatibilities 
RUN cargo test
RUN cargo build

# copies the current dir to another dir in the container
ADD . ./usr/app

# uses this dir
WORKDIR /usr/app

# todo: check /usr/app once production ready

CMD ["cargo", "run"]