[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Status](https://img.shields.io/badge/pull--request-open-blue)]()

# Canduma rust authentication server boilerplate
`A Rust authentication server with GraphQL API, Diesel, PostgreSQL session authentication and JWT`

This repository contains boilerplate rust code for getting a GraphQL prototype with JWT up and running quickly.
 
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
* Stable Toolchain: `rustup default stable`
* Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
* PostgreSQL database server or use our docker-compose.yml (require docker)

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
![Register with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-register.png?raw=true)

### Login
![Login with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-login.png?raw=true)

### Get my account
![Login with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-get-me.png?raw=true)

### Get JWT Token
![Get JWT by GraphQL with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-grahql-get-jwt.png?raw=true)

### Set Bearer JWT Token
![Set JWT Token with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/insomnia-set-bearer.png?raw=true)

### Get decoded JWT by the server (for tests purpose)
![Get JWT decoded Token by GraphQL with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-grahql-get-jwt-decoded.png?raw=true)

### Test authentication with session in GraphQL by getting all users (for tests purpose)
![Get all users by GraphQL with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-grahql-get-users.png?raw=true)

### Logout
![Logout with Insomnia](https://github.com/clifinger/canduma/blob/master/docs/images/new-insomnia-logout.png?raw=true)


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

## Build release
```sh 
cargo build --release
cd target/release
./canduma
```

## Security
### Important security considerations

We use session cookies for authentication.

__Why not JWT authentication?__

[Stop Using JWT for sessions and why your solution doesn't work](http://cryto.net/~joepie91/blog/2016/06/19/stop-using-jwt-for-sessions-part-2-why-your-solution-doesnt-work/)

The use of JWT remains secure only if you use adequate storage. 
This boilerplate is built for use in a micro-services architecture. 

JWT can be use for representing claims to be transferred between two parties.

The private key should only be on this  micro-service.
public key can be used on all other parties to decode the token.

This boilerplate provides a complete example, so we included JWT also.

### Generate RSA keys for JWT
In development mode you can keep the one in `/keys` folder.

```shell script
// private key
$ openssl genrsa -out rs256-4096-private.rsa 4096
$ openssl rsa -in rs256-4096-private.rsa -outform DER -out private_rsa_key.der

// public key
$ openssl rsa -in rs256-4096-private.rsa -pubout > rs256-4096-public.pem
$ openssl rsa -in private_rsa_key.der -inform DER -RSAPublicKey_out -outform DER -out public_rsa_key.der
```