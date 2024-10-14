
use sqlx::PgPool;

// Define the application state to hold the database connection pool
pub struct AppState {
    pub db_pool: PgPool,
}
