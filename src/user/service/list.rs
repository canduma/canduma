use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::user::model::User;
use diesel::prelude::*;

pub(crate) fn find_all_users(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(users
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<User>(conn)?)
}
