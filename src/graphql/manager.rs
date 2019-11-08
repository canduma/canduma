use crate::database::pool::PgPooledConnection;
use crate::graphql::model::{Context, Mutation, QueryRoot, Schema};
use crate::jwt::model::DecodedToken;
use crate::user::model::SlimUser;
use std::sync::Arc;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}

pub fn create_context(token: DecodedToken, user: SlimUser, pg_pool: PgPooledConnection) -> Context {
    Context {
        token,
        user,
        db: Arc::new(pg_pool),
    }
}
