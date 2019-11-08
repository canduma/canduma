use crate::graphql::handler::graphql_handler;
use actix_web::web;

pub fn route_graphql(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to_async(graphql_handler)));
}
