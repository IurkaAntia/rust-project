-- Your SQL goes here
-- migrations/<timestamp>_create_users_table/up.sql
CREATE TABLE users (
                       id INT PRIMARY KEY AUTO_INCREMENT,
                       name VARCHAR(255) NOT NULL,
                       email VARCHAR(255) NOT NULL
);
