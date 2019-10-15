extern crate dotenv;

use crate::db::{PgPooledConnection};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::*;
use crate::schema::users;
use crate::utils::identity::{make_hash, make_salt};
use crate::errors::ServiceError;
use crate::utils::jwt::create_token;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<PgPooledConnection>,
    pub email: Option<String>
}

impl juniper::Context for Context {}

#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
struct User {
    #[allow(warnings)]
    #[graphql(skip)]
    user_id: i32,
    #[graphql(name = "uuid")]
    user_uuid: Uuid,
    #[allow(warnings)]
    #[graphql(skip)]
    hash: Vec<u8>,
    #[allow(warnings)]
    #[graphql(skip)]
    salt: String,
    email: String,
    created_at: NaiveDateTime,
    name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser <'a> {
    user_uuid: Uuid,
    hash: Option<Vec<u8>>,
    salt: Option<String>,
    email: Option<&'a String>,
    created_at: NaiveDateTime,
    name: Option<&'a String>,
}

impl <'a> NewUser <'a> {
    fn new(name: &'a String, email: &'a String, password: &'a String) ->  NewUser <'a> {
        let salt = make_salt();
        let hash = make_hash(&password, &salt);
        NewUser {
            salt: Some(salt),
            hash: Some(hash.to_vec()),
            name: Some(&name),
            email: Some(&email),
            ..Default::default()
        }
    }
}

impl <'a> Default for NewUser <'a> {
    fn default() -> Self {
        Self {
            user_uuid: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            salt: None,
            email: None,
            name: None,
            hash: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: Option<String>,
}

#[derive(juniper::GraphQLObject)]
struct Token {
    bearer: Option<String>,
    user: User
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            email: Some(user.email)
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
struct RegisterInput {
    email: String,
    name: String,
    password: String,
}

#[derive(juniper::GraphQLInputObject)]
struct LoginInput {
    email: String,
    password: String,
}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context) -> Vec<User> {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &context.db;

        users
            .limit(100)
            .load::<User>(conn)
            .expect("Error loading members")
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    pub fn register(context: &Context, input: RegisterInput) -> Result<User, ServiceError> {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &context.db;

        let new_user = NewUser::new(&input.name, &input.email, &input.password);

        match diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into())
        }
    }

    pub fn login(context: &Context, input: LoginInput) -> Result<Token, ServiceError> {
        use crate::schema::users::dsl::*;
        let conn: &PgConnection = &context.db;

        let mut items = users
            .filter(email.eq(&input.email))
            .load::<User>(conn)?;

        if let Some(user) = items.pop() {
            if make_hash(&input.password, &user.salt) == user.hash {
                match create_token(input.email.as_str()) {
                    Ok(r) => return Ok(Token {
                        bearer: Some(r),
                        user
                    }),
                    Err(e) => return Err(ServiceError::Unauthorized)
                };
            }
        }
        Err(ServiceError::Unauthorized)
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}

pub fn create_context(user_email: Option<String>, pg_pool: PgPooledConnection) -> Context {
    Context {
        email: user_email,
        db: Arc::new(pg_pool),
    }
}