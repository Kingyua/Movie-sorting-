
use actix_web::web;
use crate::handlers::{add_movie, get_movies, register_user, login_user, logout_user};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movies")
            // No need to wrap IdentityMiddleware here, it's already applied in main.rs
            .route("", web::post().to(add_movie))
            .route("", web::get().to(get_movies))
    )
    .service(
        web::scope("/auth")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/logout", web::post().to(logout_user))
    );
}
