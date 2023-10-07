#[macro_use] 
extern crate rocket;
extern crate diesel;
extern crate rand;

mod consts;
mod schema;
mod diesel_funcs;
mod models;
mod rocket_files {
    pub mod enums;
    pub mod rocket_funcs;
    pub mod rocket_structs;
    pub mod routes {
        pub mod get_routes;
        pub mod post_routes;
    }
}

use rocket::fs::{FileServer, relative};
use rocket_files::rocket_structs::ChatMessage;
use rocket::tokio::sync::broadcast::{channel};
use crate::rocket_files::routes::get_routes::{redirect_to_login, get_home};
use crate::rocket_files::routes::post_routes::{post_home, login, create_account, post_message, events};

#[launch]
fn rocket() -> _ {

    rocket::build()
        .manage(channel::<ChatMessage>(1024).0)
        .mount("/", routes![create_account, login, redirect_to_login, get_home, post_home, post_message, events])
        .mount("/", FileServer::from(relative!("static/")))
}
