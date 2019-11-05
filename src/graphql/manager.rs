use crate::user::model::{SlimUser};
use crate::graphql::model::{Schema, QueryRoot, Mutation, Context};
use std::sync::Arc;
use crate::database::pool::PgPooledConnection;
use crate::jwt::model::DecodedToken;

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