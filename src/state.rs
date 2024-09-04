use std::sync::Mutex;
use crate::models::Movie;

// Define the application state to hold movies
pub struct AppState {
    pub movies: Mutex<Vec<Movie>>,
}
