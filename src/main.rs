#[macro_use]
extern crate diesel;
extern crate juniper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


use std::sync::Arc;
use actix_web::{web, Error, App, HttpResponse, HttpServer, middleware};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::serde::ser::Error as SerdeError;

use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use dotenv::dotenv;

mod handlers;
mod db;
mod models;
mod schema;
mod utils;
mod errors;

use crate::models::user::{create_schema, Schema, create_context};
use crate::db::{establish_connection, PgPool};
use crate::handlers::LoggedUser;
use std::env;

pub fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    user: LoggedUser,
    pool: web::Data<PgPool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let pg_pool = pool
            .get()
            .map_err(|e| {
                serde_json::Error::custom(e)
            })?;

        let ctx = create_context(user.email, pg_pool);

        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user))
        })
}

fn main() {
    let sys = actix::System::new("canduma");
    let mut builder = SslAcceptor::mozilla_intermediate(
        SslMethod::tls()
    ).unwrap();

    let schema = std::sync::Arc::new(create_schema());
    /*let pool = establish_connection();
    let schema_context = Context { db: pool.clone(), user: "hello".to_string() };*/

    builder
        .set_private_key_file("certs/rust.localhost.key", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("certs/rust.localhost.crt").unwrap();

    dotenv().ok();
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .data(establish_connection())
            .data(schema.clone())
            //.data(schema_context.clone())
            .wrap(middleware::Compress::default())
            .service(
                web::resource("/graphql").route(web::post().to_async(graphql))
            )
    })
        //.bind_ssl("127.0.0.1:8088", builder).unwrap().start();
        .bind(("0.0.0.0", port)).unwrap().start();
    let _ = sys.run();
}