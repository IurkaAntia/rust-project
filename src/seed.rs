use diesel::prelude::*;
use crate::schema::users;
use crate::establish_connection;
use diesel::insert_into;

pub fn run() {
    let mut connection = establish_connection();

    // Insert some sample users into the database
    let new_users = vec![
        ("Alice", "alice@example.com"),
        ("Bob", "bob@example.com"),
        ("Charlie", "charlie@example.com"),
    ];

    for user in new_users {
        // Insert users into the `users` table
        insert_into(users::table)
            .values((
                users::name.eq(user.0),
                users::email.eq(user.1),
            ))
            .execute(&mut connection)
            .expect("Error inserting users");
    }

    println!("Seed data inserted successfully!");
}
