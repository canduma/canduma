use crate::database::pool::PgPool;
use crate::graphql::manager::create_context;
use crate::graphql::model::Schema;
use actix_web::{web, Error, HttpResponse};
use futures::Future;
use juniper::http::GraphQLRequest;
use std::sync::Arc;
extern crate serde_json;
use crate::jwt::model::DecodedToken;
use crate::serde::ser::Error as SerdeError;
use crate::user::model::LoggedUser;

pub fn graphql_handler(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    user: LoggedUser,
    token: DecodedToken,
    pool: web::Data<PgPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let pg_pool = pool.get().map_err(|e| serde_json::Error::custom(e))?;

        let ctx = create_context(token, user, pg_pool);

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
