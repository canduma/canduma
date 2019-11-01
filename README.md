# Rust web server with GraphQL API, Diesel Postgres and JWT authentication

## Required
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
### Register
![Register with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/insomnia-register.png?raw=true)

### Login
![Login with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/insomnia-login.png?raw=true)

### Set Bearer JWT Token
![Set JWT Token with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/insomnia-set-bearer.png?raw=true)

### Test authentication with JWT by getting all users
![Set Token with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/insomnia-test-jwt-by-get-members.png?raw=true)

