FROM rust:latest

COPY . ./usr/app

WORKDIR /usr/app

CMD ["ls"]