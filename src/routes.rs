use actix_web::web;
use crate::handlers::{add_movie, get_movies, mark_watched, delete_movie};

// Configure routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movies")
            .route("", web::post().to(add_movie))          // Add movie
            .route("", web::get().to(get_movies))          // Get all movies
            .route("/{id}/watch", web::put().to(mark_watched)) // Mark as watched
            .route("/{id}", web::delete().to(delete_movie)),  // Delete movie
    );
}
