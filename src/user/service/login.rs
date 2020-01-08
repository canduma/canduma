use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::user::model::{SlimUser, User};
use crate::user::util::verify;
use actix_web::web;
use diesel::prelude::*;

pub fn login(
    user_email: &str,
    user_password: &str,
    pool: web::Data<Pool>,
) -> ServiceResult<SlimUser> {
    use crate::schema::users::dsl::{email, users};

    let conn = &db_connection(&pool)?;
    let user = users
        .filter(email.eq(user_email))
        .first::<User>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    if verify(&user, &user_password) {
        Ok(user.into())
    } else {
        Err(ServiceError::Unauthorized)
    }
}
