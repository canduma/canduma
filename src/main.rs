#[macro_use]
extern crate diesel;
extern crate juniper;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod database;
mod user;
mod errors;
mod schema;
mod graphql;
mod jwt;

use database::pool::establish_connection;
use std::env;
use dotenv::dotenv;
use actix_web::{HttpServer, App, middleware};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use user::route::route_user;
use crate::graphql::route::route_graphql;
use crate::graphql::manager::create_schema;


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
        .bind(("0.0.0.0", port)).unwrap().start();
    let _ = sys.run();
}