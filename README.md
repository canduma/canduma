#Rust web server with GraphQL API, Diesel Postgres and JWT authentication

##Required
* [Rustup](https://rustup.rs/)
* Nightly Toolchain: `rustup default nightly`
* Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
* PostgreSQL database server or use our docker-compose.yml (require docker)

## Installation with Docker PostgreSQL
```sh
git clone https://github.com/clifinger/canduma.git
cd canduma
docker-compose up
cp .env.example .env
diesel setup --database-url='postgres://postgres:canduma@localhost/canduma'
diesel migration run
cargo run
```
## Test the GraphQL API with Insomnia



