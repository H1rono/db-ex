# db-ex

mysql and rust in docker

## Usage

- Run main.rs: `docker compose up -d`
- Run tests: `docker compose -f docker-compose.test.yml up -d`
- Run main.rs in host machine: `DATABASE_URL=mysql://root:root@localhost:3306/sample cargo run`
    - Before execute this command, MySQL is required to be serving
