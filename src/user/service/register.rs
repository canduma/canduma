use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::user::model::{InsertableUser, SlimUser, User, UserData};
use actix_web::web;
use diesel::prelude::*;

pub fn register(user_data: UserData, pool: web::Data<Pool>) -> ServiceResult<SlimUser> {
    use crate::schema::users::dsl::users;

    let conn = &db_connection(&pool)?;
    let user: InsertableUser = user_data.into();

    let inserted_user: User = diesel::insert_into(users).values(&user).get_result(conn)?;

    Ok(inserted_user.into())
}
