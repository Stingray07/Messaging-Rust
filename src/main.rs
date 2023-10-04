#[macro_use] 
extern crate rocket;
extern crate diesel;
extern crate rand;

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
use crate::rocket_files::routes::get_routes::{redirect_to_login, get_home};
use crate::rocket_files::routes::post_routes::{post_home, login, create_account};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_account, login, redirect_to_login, get_home, post_home])
        .mount("/", FileServer::from(relative!("static/")))
}
