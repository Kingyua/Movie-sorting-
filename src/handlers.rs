
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use crate::models::Movie;
use crate::state::AppState;
use log::error;
use bcrypt::{hash, verify, DEFAULT_COST};
use actix_session::Session;
use sqlx::FromRow;

// Add a new movie
pub async fn add_movie(
    state: web::Data<AppState>,
    movie: web::Json<Movie>,
) -> impl Responder {
    let new_id = Uuid::new_v4();

    if movie.title.is_empty() {
        return HttpResponse::BadRequest().body("Title is required");
    }

    let genre = movie.genre.as_deref();
    let release_year = movie.release_year;
    let rating = movie.rating.clone();
    let watched = movie.watched.unwrap_or(false);

    match sqlx::query!(
        r#"
        INSERT INTO movies (id, title, genre, release_year, rating, watched)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_id,
        movie.title.as_str(),
        genre,
        release_year,
        rating,
        watched
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => HttpResponse::Created().json(new_id),
        Err(err) => {
            eprintln!("Error inserting movie: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to add movie: {:?}", err))
        }
    }
}

// Pagination and sorting structure
#[derive(Deserialize)]
pub struct PaginationAndSorting {
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<usize>,
    pub page: Option<usize>,
}

// Get movies with sorting and pagination
pub async fn get_movies(
    data: web::Data<AppState>, 
    query: web::Query<PaginationAndSorting>
) -> impl Responder {
    let pool: &PgPool = &data.db_pool;

    let sort_by = query.sort_by.clone().unwrap_or_else(|| "title".to_string());
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let limit = query.limit.unwrap_or(10);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let query = format!(
        "SELECT * FROM movies ORDER BY {} {} LIMIT $1 OFFSET $2", 
        sort_by, order
    );

    match sqlx::query_as::<_, Movie>(&query)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
    {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(e) => {
            error!("Failed to fetch movies: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch movies")
        }
    }
}

// Mark a movie as watched
pub async fn mark_watched(
    data: web::Data<AppState>,
    path: web::Path<Uuid>, 
    query: web::Query<bool>
) -> impl Responder {
    let id = path.into_inner();
    let watched = query.into_inner();
    let pool: &PgPool = &data.db_pool;

    match sqlx::query!(
        "UPDATE movies SET watched = $1 WHERE id = $2",
        watched,
        id
    )
    .execute(pool)
    .await
    {
        Ok(updated) => {
            if updated.rows_affected() > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            error!("Failed to mark movie as watched: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to update movie")
        }
    }
}

// Delete a movie
pub async fn delete_movie(
    data: web::Data<AppState>,
    path: web::Path<(uuid::Uuid,)>
) -> impl Responder {
    let id = path.into_inner().0;
    let pool: &PgPool = &data.db_pool;

    match sqlx::query!("DELETE FROM movies WHERE id = $1", id)
        .execute(pool)
        .await
    {
        Ok(deleted) => {
            if deleted.rows_affected() > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            error!("Failed to delete movie: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to delete movie")
        }
    }
}

// Get movies by watched status
pub async fn get_movies_by_status(
    data: web::Data<AppState>, 
    watched: web::Query<bool>
) -> impl Responder {
    let pool: &PgPool = &data.db_pool;
    let watched_status = watched.into_inner();

    match sqlx::query_as::<_, Movie>(
        "SELECT * FROM movies WHERE watched = $1",
    )
    .bind(watched_status)
    .fetch_all(pool)
    .await
    {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(e) => {
            error!("Failed to fetch movies by status: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch movies")
        }
    }
}

// Register a new user
#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password_hash: String,
}

pub async fn register_user(
    data: web::Data<AppState>, 
    user: web::Json<RegisterUser>
) -> impl Responder {
    let pool: &PgPool = &data.db_pool;

    let password_hash = match hash(&user.password_hash, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Password hashing failed: {:?}", e);
            return HttpResponse::InternalServerError().json("Password hashing failed");
        }
    };

    let new_id = Uuid::new_v4();

    match sqlx::query!(
        "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)",
        new_id, user.username, password_hash
    )
    .execute(pool)
    .await
    {
        Ok(_) => HttpResponse::Created().json("User registered"),
        Err(e) => {
            error!("Failed to register user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to register user")
        }
    }
}

// Login user
#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

// Define a struct to map the SQL query result
#[derive(FromRow)]
struct UserRow {
    id: uuid::Uuid,            // Ensure this is the correct UUID type
    password_hash: String,
}

pub async fn login_user(
    data: web::Data<AppState>,
    login_data: web::Json<LoginData>,
    session: Session,
) -> impl Responder {
    let pool: &PgPool = &data.db_pool; // Accessing the database pool from AppState

    // SQL query to fetch user details
    let query = "SELECT id, password_hash FROM users WHERE username = $1";

    // Execute the query
    match sqlx::query_as::<_, UserRow>(query) // Use query_as to map the result to UserRow
        .bind(&login_data.username) // Bind username directly
        .fetch_one(pool)
        .await
    {
        Ok(user) => {
            // Accessing fields correctly
            let user_id = user.id; // Accessing user id
            let user_password_hash = user.password_hash; // Accessing user password hash
            
            // Verify password
            if verify(&login_data.password, &user_password_hash).unwrap_or(false) {
                // Insert the user id into the session
                session.insert("user_id", user_id).unwrap();
                HttpResponse::Ok().json("Login successful") // Successful login response
            } else {
                HttpResponse::Unauthorized().json("Invalid credentials") // Invalid password response
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("User not found"), // User does not exist response
    }
}

// Logout user
pub async fn logout_user(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json("Logout successful")
}
