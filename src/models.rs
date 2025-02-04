use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Define the User struct (model)
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
