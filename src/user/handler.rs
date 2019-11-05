use actix_web::{HttpResponse, web, FromRequest, Error, HttpRequest};
use actix_web::error::BlockingError;
use crate::database::pool::PgPool;
use futures::Future;
use crate::errors::ServiceError;
use crate::user::model::{UserData, SlimUser, AuthData, LoggedUser};
use crate::user::manager::{user_manager_register, user_manager_login};
use actix_identity::Identity;
use actix_web::dev::Payload;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Result<LoggedUser, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {

        if let Some(identity) = Identity::from_request(req, pl)?.identity() {
            let user: LoggedUser = serde_json::from_str(&identity)?;
            return Ok(user);
        }

        Ok(LoggedUser {email: None})
    }
}

pub fn user_handler_register(
    user_data: web::Json<UserData>,
    pool: web::Data<PgPool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || {
        user_manager_register(
            user_data.into_inner(),
            pool,
        )
    })
        .then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(&user)),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        })
}

pub fn user_handler_login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<PgPool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || user_manager_login(auth_data.into_inner(), pool)).then(
        move |res: Result<SlimUser, BlockingError<ServiceError>>| match res {
            Ok(user) => {
                let user_string = serde_json::to_string(&user).unwrap();
                id.remember(user_string);
                Ok(HttpResponse::Ok().finish())
            }
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        },
    )
}

pub fn user_handler_me(logged_user: LoggedUser) -> HttpResponse {
    if logged_user.email == None {
        return HttpResponse::Unauthorized().json(ServiceError::Unauthorized);
    }
    HttpResponse::Ok().json(logged_user)
}

pub fn user_handler_logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}