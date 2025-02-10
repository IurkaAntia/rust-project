use actix_web::web;
use crate::controllers::users::{get_users, create_user}; // Correct path

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(create_user);
}
