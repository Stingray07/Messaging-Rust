#![allow(unused_assignments)]

use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use std::env;
use crate::schema::users::{self};
use crate::schema::sessions::{self};
use chrono::{NaiveDate, Utc};
use rand::Rng;
use crate::rocket_files::rocket_structs::CreateAccount;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_users(account: &mut CreateAccount) -> Result<(), Error> {
    let mut conn =  establish_connection();
    let current_date = Utc::now().date_naive();
    let option_current_date: Option<NaiveDate> = Some(current_date);

    match diesel::insert_into(users::table)
        .values((
            users::username.eq(&account.username),
            users::acc_password.eq(&account.password),
            users::first_name.eq(&account.first_name),
            users::last_name.eq(&account.last_name),
            users::creation_date.eq(option_current_date)
        ))
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn user_exists(username_param: &str, password_param: Option<&String>) -> bool {
    let mut conn = establish_connection();

    let mut check: Result<String, diesel::result::Error> = Ok(String::new());

    if let Some(password) = password_param {
        check = users::table
            .select(users::username)
            .filter(users::username.eq(username_param))
            .filter(users::acc_password.eq(password))
            .first(&mut conn);
    } else {
        check = users::table
            .select(users::username)
            .filter(users::username.eq(username_param))
            .first(&mut conn);
    }

    match check {
        Ok(_) => {
            true
        }
        Err(e) => {
            println!("ERROR : {}", e);
            false
        }
    }
}

pub fn generate_session_id() -> String {
    let mut rng = rand::thread_rng();
    let sess_id: String = (0..32).map(|_| rng.gen_range(0..16).to_string()).collect();
    sess_id
}

pub fn insert_session(user_id_param: &i32, session_id_param: &String) -> Result<(), diesel::result::Error> {
    let mut conn = establish_connection();

    match diesel::insert_into(sessions::table)
        .values((
            sessions::user_id.eq(user_id_param),
            sessions::session_id.eq(session_id_param)))
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_session_id_from_db(user_id_param: &i32) -> Result<String, diesel::result::Error> {
    let mut conn = establish_connection();

    let join = sessions::table.left_join(users::table);

    let query = join
        .select(sessions::session_id)
        .filter(sessions::user_id.eq(user_id_param))
        .first(&mut conn);

    query
}

pub fn get_user_id(username_param: &str) -> Result<i32, diesel::result::Error> {

    let mut conn = establish_connection();
    let join = users::table.left_join(sessions::table);

    let id = join
        .select(users::user_id)
        .filter(users::username.eq(username_param))
        .first(&mut conn);
    
    id
}

pub fn delete_session_id(session_id_param: &String) -> Result<() , diesel::result::Error>{
    let mut conn = establish_connection();

    match diesel::delete(sessions::table.filter(sessions::session_id.eq(session_id_param)))
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}