use crate::schema::*;
use crate::user::util::{make_hash, make_salt};
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    pub user_id: i32,
    pub user_uuid: Uuid,
    #[graphql(skip)]
    pub hash: Vec<u8>,
    #[graphql(skip)]
    pub salt: String,
    pub email: String,
    pub role: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub user_uuid: Uuid,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub user_uuid: Uuid,
    pub email: String,
    pub role: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            name,
            email,
            password,
            ..
        } = user_data;

        let salt = make_salt();
        let hash = make_hash(&password, &salt).to_vec();
        Self {
            user_uuid: Uuid::new_v4(),
            email,
            hash,
            created_at: chrono::Local::now().naive_local(),
            salt,
            name,
            role: "user".to_owned(),
        }
    }
}
impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            user_uuid,
            email,
            role,
            ..
        } = user;

        Self {
            user_uuid,
            email,
            role,
        }
    }
}
