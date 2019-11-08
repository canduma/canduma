#[macro_use]
extern crate diesel;
extern crate juniper;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod database;
mod errors;
mod graphql;
mod jwt;
mod schema;
mod user;

use crate::graphql::manager::create_schema;
use crate::graphql::route::route_graphql;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, App, HttpServer};
use database::pool::establish_connection;
use dotenv::dotenv;
use std::env;
use user::route::route_user;

fn main() {
    dotenv().ok();
    let sys = actix::System::new("canduma");
    let schema = std::sync::Arc::new(create_schema());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    HttpServer::new(move || {
        App::new()
            .data(establish_connection())
            .data(schema.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(user::util::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            .wrap(middleware::Compress::default())
            .configure(route_user)
            .configure(route_graphql)
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .start();
    let _ = sys.run();
}
