[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)

# Rust authentication server boilerplate

This is based on [canduma](https://github.com/clifinger/canduma).
The main differences with canduma are:

- use async/await,
- use actix-web version 2,
- use anyhow + thiserror in place of failure,
- structopt

`A Rust authentication server with GraphQL API, Diesel, PostgreSQL session authentication and JWT`

This repository contains boilerplate rust code for getting a GraphQL prototype with JWT up and running quickly.

It uses [actix-web](https://actix.rs/), [Juniper](https://graphql-rust.github.io/juniper/current/), [Diesel](http://diesel.rs/) and [jsonwebtoken](https://docs.rs/jsonwebtoken)

## Collection of major crates used

- actix - [link](https://actix.rs/)
- actix-web - [link](https://docs.rs/actix-web/)
- diesel - [link](http://diesel.rs/)
- juniper - [link](https://graphql-rust.github.io/juniper/current/)
- chrono - [link](https://docs.rs/chrono/)
- serde_json - [link](https://docs.serde.rs/serde_json/)
- argon2rs - [link](https://github.com/bryant/argon2rs)
- jsonwebtoken - [link](https://docs.rs/jsonwebtoken)
- anyhow - [link](https://github.com/dtolnay/anyhow)
- thiserror - [link](https://github.com/dtolnay/thiserror)
- shrinkwraprs - [link](https://docs.rs/shrinkwraprs/)
- spectral - [link](https://github.com/cfrancia/spectral)

## Required

- [Rustup](https://rustup.rs/)
- Stable Toolchain: `rustup default stable`
- Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
- PostgreSQL database server or use our docker-compose.yml (require docker)

## Getting Started

```sh
git clone https://github.com/s-ted/canduma-async
cd canduma-async
docker-compose up
cp .env.example .env
diesel setup --database-url='postgres://postgres:canduma@localhost/canduma'
diesel migration run
cargo run
```

## Test the GraphQL API with VScode REST Client

[VScode plugin](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

See TEST.http file.

## Build release

```sh
cargo build --release
cd target/release
./canduma-async
```

## Security

### Important security considerations

We use session cookies for authentication.

**Why not JWT authentication?**

[Stop Using JWT for sessions and why your solution doesn't work](http://cryto.net/~joepie91/blog/2016/06/19/stop-using-jwt-for-sessions-part-2-why-your-solution-doesnt-work/)

The use of JWT remains secure only if you use adequate storage.
This boilerplate is built for use in a micro-services architecture.

JWT can be use for representing claims to be transferred between two parties.

The private key should only be on this micro-service.
public key can be used on all other parties to decode the token.

This boilerplate provides a complete example, so we included JWT also.

### Generate RSA keys for JWT

In development mode you can keep the one in `/keys` folder.

```shell script
// private key
$ openssl genrsa -out rs256-4096-private.rsa 4096

// public key
$ openssl rsa -in rs256-4096-private.rsa -pubout > rs256-4096-public.pem
```
