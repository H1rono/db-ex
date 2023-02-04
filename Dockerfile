FROM rust:1.67-buster

WORKDIR /dbexample-rs
COPY . .

RUN cargo build

CMD [ "cargo", "run" ]
