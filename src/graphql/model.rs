use crate::cli_args::Opt;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::jwt::model::{DecodedToken, Token};
use crate::user::model::{LoggedUser, SlimUser, User, UserData};
use crate::user::service as user;
use crate::user::service::token::ClaimsResponse;
use diesel::PgConnection;
use juniper::Context as JuniperContext;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Context {
    pub opt: Opt,
    pub db: Arc<PooledConnection>,
    pub user: LoggedUser,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(token: DecodedToken, user: LoggedUser, pool: PooledConnection, opt: Opt) -> Self {
        Self {
            opt,
            token,
            user,
            db: Arc::new(pool),
        }
    }
}

pub(crate) struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<User>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        crate::user::has_role(&context.user, "user")?;

        user::list::find_all_users(&context, limit, offset)
    }

    pub fn generate_token(context: &Context) -> ServiceResult<Token> {
        user::token::generate(&context)
    }

    pub fn decode_token(context: &Context) -> ServiceResult<&ClaimsResponse> {
        user::token::decode(&context)
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    pub fn register(context: &Context, data: UserData) -> ServiceResult<SlimUser> {
        use crate::user::service::register::create_user;
        let conn: &PgConnection = &context.db;

        Ok(create_user(data, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
