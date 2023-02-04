# db-ex

mysql and rust in docker

## MySQL

- start server: `docker compose up -d`

## Rust

- using outside docker
    - load env: `export DATABASE_URL=mysql://root:root@localhost:3306/sample`
    - run: `cargo run`
