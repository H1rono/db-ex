FROM rust:1.66-buster

WORKDIR /dbexample-rs
COPY . .

RUN cargo install --path .

CMD [ "dbexample-rs" ]
