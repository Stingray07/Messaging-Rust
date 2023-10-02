#![allow(unused_assignments)]

use rocket::http::CookieJar;
use crate::rocket_files::enums::NotFoundError;

pub const ACCOUNT_CREATION_SUCCESS_MESSAGE: &str = "ACCOUNT CREATION SUCCESSFUL";
pub const ACCOUNT_CREATION_FAILURE_MESSAGE: &str = "ACCOUNT CREATION FALIED";
pub const ACCOUNT_CREATION_FAILURE_USER_EXISTS: &str = "ACCOUNT CREATION FAILED: User already exists";
pub const ACCOUNT_LOGIN_SUCCESS_MESSAGE: &str = "ACCOUNT LOGIN SUCCESSFUL";
pub const ACCOUNT_LOGIN_FAILURE_USER_NOT_FOUND_MESSAGE: &str = "ACCOUNT LOGIN UNSUCCESSFUL: Username not found";
pub const ACCOUNT_LOGIN_FAILURE_SESSION_FAIL: &str = "ACCOUT LOGIN UNSUCCESSFUL: Session insert fail";
pub const SESSION_ID_NOT_FOUND_COOKIE: &str = "SESSION NOT FOUND: Session not found from cookie";
pub const USER_ID_NOT_FOUND_COOKIE: &str = "USER_ID NOT FOUND: User ID not fround from cookie";

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