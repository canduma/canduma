extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::{EmptyMutation, RootNode};

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
struct Member {
    id: i32,
    name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

#[juniper::object(description = "A member of a team")]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn knockouts(&self) -> i32 {
        self.knockouts
    }

    pub fn team_id(&self) -> i32 {
        self.team_id
    }
}

pub struct QueryRoot;


#[juniper::object]
impl QueryRoot {
    fn members() -> Vec<Member> {
        use crate::schema::members::dsl::*;
        let connection = establish_connection();
        members
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error loading members")
    }
    fn teams() -> Vec<Team> {
        use crate::schema::teams::dsl::*;
        let connection = establish_connection();
        teams
            .limit(10)
            .load::<Team>(&connection)
            .expect("Error loading teams")
    }
}

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[juniper::object(description = "A team of members")]
impl Team {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn members(&self) -> Vec<Member> {
        use crate::schema::members::dsl::*;
        let connection = establish_connection();
        members
            .filter(team_id.eq(self.id))
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error loading members")
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}