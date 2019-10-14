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
    pub email: String
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

impl User {
    fn new(name: &str, email: &str, password: &str) -> NewUser {
        let salt = make_salt();
        let hash = make_hash(&password, &salt);
        NewUser {
            salt: salt.to_string(),
            hash: hash.to_vec(),
            name: name.to_string(),
            email: email.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    user_uuid: Uuid,
    hash: Vec<u8>,
    salt: String,
    email: String,
    created_at: NaiveDateTime,
    name: String,
}

impl Default for NewUser {
    fn default() -> Self {
        Self {
            user_uuid: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            salt: "".to_string(),
            email: "".to_string(),
            name: "".to_string(),
            hash: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

#[derive(juniper::GraphQLObject)]
struct Token {
    bearer: String
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            email: user.email
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

        let new_user = User::new(&input.name, &input.email, &input.password);

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
                        bearer: context.email.to_string()
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

pub fn create_context(user_email: &str, pg_pool: PgPooledConnection) -> Context {
    Context {
        email: user_email.to_string(),
        db: Arc::new(pg_pool),
    }
}