use crate::database::pool::PgPooledConnection;
use crate::errors::ServiceError;
use crate::jwt::model::{DecodedToken, HumanClaims, Token};
use crate::user::manager::{user_manager_get_all, user_manager_get_decode, user_manager_get_jwt};
use crate::user::model::{SlimUser, User};
use juniper::Context as JuniperContext;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<PgPooledConnection>,
    pub user: SlimUser,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context) -> Result<Vec<User>, ServiceError> {
        user_manager_get_all(&context)
    }
    pub fn token(context: &Context) -> Result<Token, ServiceError> {
        user_manager_get_jwt(&context)
    }
    pub fn decode(context: &Context) -> Result<&HumanClaims, ServiceError> {
        user_manager_get_decode(&context)
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

pub type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;
