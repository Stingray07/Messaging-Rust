#![allow(unused_assignments)]

use rocket::http::CookieJar;
use crate::rocket_files::enums::NotFoundError;
use super::rocket_structs::{ResponseStruct, SharedData};
use rocket::State;

pub fn format_credentials(username: Option<&str>, 
    password: Option<&str>, 
    first_name: Option<&str>, 
    last_name: Option<&str>
) {
    match (username, password, first_name, last_name) {
        (Some(username), Some(password), Some(first_name), Some(last_name)) => {
            println!("CREATE ACCOUNT");
            println!("USERNAME : {}", username);
            println!("PASSWORD : {}", password);
            println!("FIRST NAME : {}", first_name);
            println!("LAST NAME : {}", last_name);
        }
        (Some(username), Some(password), _, _) => {
            println!("LOGIN");
            println!("USERNAME : {}", username);
            println!("PASSWORD : {}", password);
        }
        _ => {
            println!("Missing required parameters for auth");
        }
    }
}

pub fn get_session_id_from_cookie(cookies: &CookieJar<'_>) -> Result<String, NotFoundError> {
    let mut session_id_from_cookie = String::from("");

    match cookies.get("session_id") {
        Some(session) => {
            session_id_from_cookie = session.value().to_string();
            println!("SESSION_ID FROM COOKIE = {:?}", session_id_from_cookie);
            Ok(session_id_from_cookie)
        },
        None => Err(NotFoundError::SessionIdNotFoundFromCookie)
    }
}

pub fn get_user_id_from_cookie(cookies: &CookieJar<'_>) -> Result<String, NotFoundError> {
    let mut user_id_from_cookie = String::from("");

    match cookies.get("user_id") {
        Some(id) => {
            user_id_from_cookie = id.value().to_string();
            println!("USER ID FROM COOKIE = {:?}", user_id_from_cookie);
            Ok(user_id_from_cookie)
        },
        None => Err(NotFoundError::UserIdNotFoundFromCookie)
    }

}

pub fn get_username_from_cookie(cookies: &CookieJar<'_>) -> Result<String, NotFoundError> {
    let mut username_from_cookie = String::from("");

    match cookies.get("username") {
        Some(username) => {
            username_from_cookie = username.value().to_string();
            println!("USERNAME FROM COOKIE = {:?}", username_from_cookie);
            Ok(username_from_cookie)
        }
        None => Err(NotFoundError::UsernameNotFoundFromCookie)
    }
}

pub fn create_response_struct() -> ResponseStruct {
    ResponseStruct { response: (String::from("")), message: String::from("") }
}

pub fn modify_response_stuct(response_struct: &mut ResponseStruct,
    response: String,
    message: String
) ->  &mut ResponseStruct {
    response_struct.response = response;
    response_struct.message = message;
    response_struct
}
