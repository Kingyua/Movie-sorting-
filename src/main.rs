/*To create a simple media tracker using rust - actix */
/*Features:
CRUD Operations: Add and delete movies.
Watched Status: Update the movie status when watched.
Filtering: Retrieve movies based on their watched status. */

use actix_web::{web, App, HttpServer};
//modules 
use std::sync::Mutex;
mod handlers;
mod models;
mod state;
mod routes;

use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state
    let app_state = web::Data::new(AppState {
        movies: Mutex::new(vec![]),
    });

    // Set up the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::configure)  // Configure routes from the routes module
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
