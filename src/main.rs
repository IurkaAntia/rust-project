use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection; // Import MysqlConnection from Diesel
use dotenv::dotenv;
use std::env;

mod models; // Import your models
mod schema;
mod seed;
// Import your schema

// Establish connection to MySQL
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok(); // Load environment variables from .env
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    MysqlConnection::establish(&database_url)
        .expect("Error connecting to the database")
}

// Define route handler for getting users
async fn get_users() -> HttpResponse {
    let mut connection = establish_connection();

    // Query users from the database
    use crate::schema::users::dsl::*;
    let results = users
        .load::<models::User>(&mut connection) // Use models::User
        .expect("Error loading users");

    HttpResponse::Ok().json(results) // Return users as JSON response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix web server
    seed::run();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_users)) // Define route for fetching users
    })
        .bind("127.0.0.1:8080")? // Bind to localhost
        .run()
        .await
}
