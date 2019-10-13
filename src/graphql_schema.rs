extern crate dotenv;
use crate::db::PgPool;
use diesel::prelude::*;
use juniper::{RootNode};
use uuid::Uuid;
use chrono::*;
use crate::schema::users;
use crate::utils::{make_hash, make_salt};
use crate::error::ServiceError;

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
struct User {
    #[allow(warnings)]
    #[graphql(skip)]
    user_id: i32,
    #[graphql(name="uuid")]
    user_uuid: Uuid,
    #[allow(warnings)]
    #[graphql(skip)]
    hash: Vec<u8>,
    #[allow(warnings)]
    #[graphql(skip)]
    salt: String,
    email: String,
    created_at: NaiveDateTime,
    name: String
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser {
    user_uuid: Uuid,
    hash: Vec<u8>,
    salt: String,
    email: String,
    created_at: NaiveDateTime,
    name: String,
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
        let connection = context.db.get()
            .unwrap_or_else(|_| panic!("Error connecting"));

        users
            .limit(100)
            .load::<User>(&connection)
            .expect("Error loading members")
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    pub fn register(context: &Context, input: RegisterInput) -> Result<User, ServiceError> {
        use crate::schema::users::dsl::*;
        let connection = context.db.get()
            .unwrap_or_else(|_| panic!("Error connecting"));

        let salt_for_password = make_salt();
        let encoded_password = make_hash(&input.password, &salt_for_password);

        let mut user = NewUser {
            user_uuid: Uuid::new_v4(),
            salt: salt_for_password,
            hash: encoded_password,
            created_at: Utc::now().naive_utc(),
            email: input.email,
            name: input.name,
        };

        match diesel::insert_into(users)
            .values(&user)
            .get_result(&connection) {
            Ok(user) => Ok(user),
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    pub fn login(context: &Context, input: LoginInput) -> Result<User, ServiceError> {
        use crate::schema::users::dsl::*;
        let connection = context.db.get()
            .unwrap_or_else(|_| panic!("Error connecting"));

        let mut items = users
            .filter(email.eq(&input.email))
            .load::<User>(&connection)?;

        if let Some(user) = items.pop() {
            if make_hash(&input.password, &user.salt) == user.hash {
                    return Ok(user.into());
            }
        }
        Err(ServiceError::Unauthorized)
    }
}

pub type Schema = RootNode<'static, QueryRoot, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation)
}