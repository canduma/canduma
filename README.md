[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Status](https://img.shields.io/badge/pull--request-open-blue)]()

# Canduma rust Graphql

`A Rust authentication server with GraphQL API, Diesel, PostgreSQL session authentication and JWT`

This repository contains a GraphQL server with JWT up and running quickly.

It uses [actix-web](https://actix.rs/), [Juniper](https://graphql-rust.github.io/juniper/current/),
[Diesel](http://diesel.rs/) and [jsonwebtoken](https://docs.rs/jsonwebtoken)

Your own pull requests are welcome!

## Benchmarks with insert into PostgreSQL

```shell script
▶ ./bombardier -c 125 -n 10000000 http://localhost:3000/graphql -k -f body --method=POST -H "Content-Type: application/json" -s
Bombarding http://localhost:3000/graphql with 10000000 request(s) using 125 connection(s)

10000000 / 10000000 [===========================================================================] 100.00% 28777/s 5m47s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     28788.66    2183.47   34605.95
  Latency        4.32ms   543.07us   110.95ms
  HTTP codes:
    1xx - 0, 2xx - 10000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    20.75MB/s
```

## Collection of major crates used in Canduma

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

## Required

- [Rustup](https://rustup.rs/)
- Stable Toolchain: `rustup default stable`
- Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
- PostgreSQL database server or use our docker-compose.yml (require docker)

## Getting Started

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

![Register with Insomnia](docs/images/new-insomnia-register.png)

### Login

![Login with Insomnia](docs/images/new-insomnia-login.png)

### Get my account

![Login with Insomnia](docs/images/new-insomnia-get-me.png)

### Get JWT Token

![Get JWT by GraphQL with Insomnia](docs/images/new-insomnia-grahql-get-jwt.png)

### Set Bearer JWT Token

![Set JWT Token with Insomnia](docs/images/insomnia-set-bearer.png)

### Get decoded JWT by the server (for tests purpose)

![Get JWT decoded Token by GraphQL with Insomnia](docs/images/new-insomnia-grahql-get-jwt-decoded.png)

### Test authentication with session in GraphQL by getting all users (for tests purpose)

![Get all users by GraphQL with Insomnia](docs/images/new-insomnia-grahql-get-users.png)

### Logout

![Logout with Insomnia](docs/images/new-insomnia-logout.png)

### Raw code for Insomnia

```text
############ GraphQL Queries ############
query usersQuery {
  users {
    name
    userUuid
    email
    createdAt
  }
}

query tokenQuery {
  token {
    bearer
  }
}

query decodeTokenQuery {
  decode {
    email
    iss
    iat
    exp
    sub
  }
}

```

## Test the GraphQL API with VScode REST Client

[VScode plugin](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

See / open TEST.http file in vscode.

## Build release

```sh
cargo build --release
cd target/release
./canduma
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

### Logging

Logging controlled by middleware::Logger [actix.rs](https://actix.rs/docs/errors/)

To enable debug logging set `RUST_LOG=debug` in `.env`

### Testing

#### Initialization

First run `yarn` or `npm install` to get all required packages

#### npm run test

To run you can use `npm run test` or `yarn test`.

The testing system designed to automatically build `canduma` offline and start in `tests/jest.beforeall.js`
We starting `canduma` in order to capture output from both rust and js code using `testci` target

#### npm run testci

```bash
$ npm run testci

> canduma@ testci /home/olexiyb/b100pro/canduma
> cross-env RUST_LOG=debug DEBUG=canduma:* NODE_ENV=test jest

Determining test suites to run...
$ killall canduma
canduma: no process found

$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
  canduma:jest.beforeall.js build = { status: 0, signal: null, output: [ null, null, null ], pid: 2447, stdout: null, stderr: null } +0ms

$  target/debug/canduma
[2020-04-02T18:17:19Z INFO  actix_server::builder] Starting 24 workers
[2020-04-02T18:17:19Z INFO  actix_server::builder] Starting server on 0.0.0.0:4000
Listening on 0.0.0.0:4000
started API 

  canduma:user.test.js /user/me body='Unauthorized' text="Unauthorized" +0ms

...
[2020-04-02T18:17:22Z DEBUG canduma::user::handler] user_string={"user_uuid":"f7cfa71e-096e-44d0-ae4f-7d16dd9e4baf","email":"email1@nowhere.com","role":"bad_role"}
  canduma:user.test.js /graphql body={ data: null, errors: [ { message: 'Unauthorized', locations: [Array], path: [Array], extensions: [Object] } ] } +292ms
 PASS  tests/user.test.js
 
...
```

In example above you see output from jest tests as well as from rust code `debug!("user_string={}", user_string);`

#### CLion

I also highly recommend to use CLion as a dev tool.
I allows to run all tests or individual with single click and analyze logs

![alt text](docs/images/clion-1.png)
