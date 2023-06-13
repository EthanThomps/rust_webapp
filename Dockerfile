FROM rust:latest

COPY . ./usr/app

## backend container + frontend container = ??

WORKDIR /usr/app

CMD ["ls"]