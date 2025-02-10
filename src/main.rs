use actix_web::{App , HttpServer};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection; // Import MysqlConnection from Diesel
use dotenv::dotenv;
use std::env;
mod models; // Import your models
mod schema;
mod controllers;
// mod seed;
mod routes;
mod db;
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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure) // Define route for fetching users
    })
        .bind("127.0.0.1:8080")? // Bind to localhost
        .run()
        .await
}
