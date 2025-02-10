use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::{User, NewUser};  // Use NewUser for inserts
use crate::schema::users::dsl::*;
use diesel::insert_into;

// Get all users
#[get("/users")]
pub async fn get_users() -> impl Responder {
    let connection = &mut establish_connection();

    match users.load::<User>(connection) {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching users"),
    }
}

// Create a new user
#[post("/users")]
pub async fn create_user(user_data: web::Json<NewUser>) -> impl Responder {
    let connection = &mut establish_connection();

    let new_user = NewUser {
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };

    let result = insert_into(users)
        .values(&new_user)
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Created().json(new_user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert user"),
    }
}
