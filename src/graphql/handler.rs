use crate::cli_args::Opt;
use crate::database::{db_connection, Pool};
use crate::graphql::model::{Context, Schema};
use crate::jwt::model::DecodedToken;
use crate::user::model::LoggedUser;
use actix_web::{error, web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub(super) async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    user: LoggedUser,
    token: DecodedToken,
    pool: web::Data<Pool>,
    opt: web::Data<Opt>,
) -> Result<HttpResponse, Error> {
    let db_pool = db_connection(&pool)?;

    let opt = opt.into_inner().as_ref().clone();
    let ctx = Context::new(token, user, db_pool, opt);

    let res = data.execute(&st, &ctx);
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub(super) fn graphiql(opt: web::Data<Opt>) -> HttpResponse {
    let html = graphiql_source(&format!("http://127.0.0.1:{}/graphql", opt.port));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
