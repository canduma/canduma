#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod cli_args;
mod database;
mod errors;
mod graphql;
mod jwt;
mod schema;
mod user;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer, web};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let schema = std::sync::Arc::new(crate::graphql::model::create_schema());
    let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = chrono::Duration::hours(i64::from(opt.auth_duration_in_hour));
    let port = opt.port;
    let pool = web::Data::new(database::pool::establish_connection(opt.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .data(schema.clone())
            .data(opt.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&domain)
                    .max_age_time(auth_duration)
                    .secure(secure_cookie),
            ))
            .configure(user::route)
            .configure(graphql::route)
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run();

    eprintln!("Listening on 0.0.0.0:{}", port);

    server.await
}
