#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;
mod database;
mod errors;
mod graphql;
mod jwt;
mod schema;
mod user;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let schema = std::sync::Arc::new(crate::graphql::model::create_schema());
    let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = time::Duration::hours(i64::from(opt.auth_duration_in_hour));
    let port = opt.port;
    let pool = database::pool::establish_connection(opt.clone());

    let server = HttpServer::new(move || {
        // prevents double Arc
        let schema: web::Data<graphql::model::Schema> = schema.clone().into();
        
        App::new()
            .data(pool.clone())
            .app_data(schema)
            .data(opt.clone())
            .wrap(Logger::default())
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
