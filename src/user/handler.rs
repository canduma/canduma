use crate::database::Pool;
use crate::errors::ServiceError;
use crate::user::model::{LoggedUser, SlimUser, UserData};
use crate::user::service as user;
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();

        let slim_user = if let Some(identity) = identity {
            match serde_json::from_str::<SlimUser>(&identity) {
                Err(e) => return futures::future::err(e.into()),
                Ok(y) => Ok(Some(y)),
            }
        } else {
            Ok(None)
        };

        futures::future::ready(slim_user.map(LoggedUser))
    }
}

pub async fn register(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user::register(user_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
}

#[derive(Debug, Deserialize)]
pub(super) struct LoginQuery {
    pub email: String,
    pub password: String,
}

pub(super) async fn login(
    auth_data: web::Json<LoginQuery>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    user::login(&auth_data.email, &auth_data.password, pool).and_then(|res| {
        let user_string =
            serde_json::to_string(&res).map_err(|_| ServiceError::InternalServerError)?;
        debug!("user_string={}", user_string);
        id.remember(user_string);
        Ok(HttpResponse::Ok().json(res))
    })
}

pub fn me(logged_user: LoggedUser) -> HttpResponse {
    match logged_user.0 {
        None => HttpResponse::Unauthorized().json(ServiceError::Unauthorized),
        Some(user) => HttpResponse::Ok().json(user),
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}
