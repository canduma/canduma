# Canduma rust server boilerplate
`A Rust web server with GraphQL API, Diesel, PostgreSQL and JWT authentication.`

This repository contains boilerplate rust code for getting a GraphQL prototype with JWT up and running quickly.
 
It uses [actix-web](https://actix.rs/), [Juniper](https://graphql-rust.github.io/juniper/current/), 
[Diesel](http://diesel.rs/) and [jsonwebtoken](https://docs.rs/jsonwebtoken)

Your own pull requests are welcome!

## Collection of major crates used in Canduma
* actix - [link](https://actix.rs/)
* actix-web - [link](https://docs.rs/actix-web/)
* diesel - [link](http://diesel.rs/)
* juniper - [link](https://graphql-rust.github.io/juniper/current/)
* chrono - [link](https://docs.rs/chrono/)
* serde_json - [link](https://docs.serde.rs/serde_json/)
* argon2rs - [link](https://github.com/bryant/argon2rs)
* jsonwebtoken - [link](https://docs.rs/jsonwebtoken)

## Required
* [Rustup](https://rustup.rs/)
* Nightly Toolchain: `rustup default nightly`
* Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
* PostgreSQL database server or use our docker-compose.yml (require docker)

## Getting Started with Docker PostgreSQL
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

