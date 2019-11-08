use crate::user::handler::{
    user_handler_login, user_handler_logout, user_handler_me, user_handler_register,
};
use actix_web::web;

pub fn route_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("/register").route(web::post().to_async(user_handler_register)))
            .service(web::resource("/login").route(web::post().to_async(user_handler_login)))
            .service(web::resource("/logout").route(web::get().to_async(user_handler_logout)))
            .service(web::resource("/me").route(web::get().to_async(user_handler_me))),
    );
}
