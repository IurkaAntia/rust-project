use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]  // Use Diesel's table reference
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]  // Add this line to ensure Diesel recognizes the table
pub struct NewUser {
    pub name: String,
    pub email: String,
}
