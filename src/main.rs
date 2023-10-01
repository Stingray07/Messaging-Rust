#[macro_use] 
extern crate rocket;
extern crate diesel;
extern crate rand;

mod schema;
mod models;
mod rocket_structs;
mod rocket_routes;
mod rocket_funcs;
mod err;

use rocket::fs::{FileServer, relative};
use crate::rocket_routes::*;

#[launch]
fn rocket() -> _ {
    
    rocket::build()
        .mount("/", routes![ test, create_account, login, redirect_to_login, get_home])
        .mount("/", FileServer::from(relative!("static/")))
}
