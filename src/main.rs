
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::PgPool;
use actix_identity::IdentityMiddleware; // Import IdentityMiddleware
use actix_session::{SessionMiddleware, storage::CookieSessionStore}; // Import for session management
use actix_web::cookie::Key; // Import for cookie key management
use dotenv::dotenv; // Import dotenv for environment variables
use std::env;

mod handlers;
mod models;
mod state;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve DATABASE_URL from the environment or fail with an error message
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment or .env file");

    // Set up the PostgreSQL connection pool, with proper error handling
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create a connection pool");

    // Initialize application state
    let app_state = web::Data::new(state::AppState {
        db_pool,
    });

    // Generate a secret key for signing session cookies
    let secret_key = Key::generate(); // Generate a random key

    // Initialize the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(), // Using cookie session store
                secret_key.clone(), // Use the secret key for signing
            ))
            .wrap(IdentityMiddleware::default()) // Use IdentityMiddleware directly
            .configure(routes::configure) // Configure your routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
