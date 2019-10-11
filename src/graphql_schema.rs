extern crate dotenv;

use crate::db::PgPool;
use diesel::prelude::*;
use juniper::{RootNode, FieldResult};
use uuid::Uuid;
use chrono::*;
use crate::schema::users;

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
    #[warn(dead_code)]
    #[graphql(name="uuid")]
    user_uuid: Uuid,
    #[allow(warnings)]
    #[graphql(skip)]
    hash: Option<String>,
    #[warn(dead_code)]
    email: String,
    created_at: NaiveDateTime,
    name: String
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser {
    user_uuid: Uuid,
    hash: Option<String>,
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
    pub fn register(context: &Context, input: RegisterInput) -> FieldResult<User> {
        use crate::schema::users::dsl::*;
        let connection = context.db.get()
            .unwrap_or_else(|_| panic!("Error connecting"));

        let user = NewUser {
            user_uuid: Uuid::new_v4(),
            hash: Some("edb01e159a4e3f3134861207f5fc5087".to_string()),
            created_at: Utc::now().naive_utc(),
            email: "contact.lenne2@gmail.com".to_string(),
            name: "Julien Lenne2".to_string(),
        };

        let inserted_user = diesel::insert_into(users)
            .values(&user)
            .get_result(&connection)?;

        Ok(inserted_user)
    }
}

pub type Schema = RootNode<'static, QueryRoot, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation)
}