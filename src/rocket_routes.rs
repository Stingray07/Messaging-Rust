use crate::rocket_structs::{CreateAccount, LoginAccount, ResponseStruct, StatusStruct};
use crate::rocket_funcs as funcs;
use crate::err::BadJSONData;
use chrono::{NaiveDate, Utc};
use rocket::serde::json::Json;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use message::*;
use rocket::fs::NamedFile;
use std::path::Path;

const HOME_PATH: &str = "static/home.html";

#[post("/create_account.html", data = "<new_account>")]
pub fn create_account(new_account: Json<CreateAccount>) -> Json<ResponseStruct> {

    let account = new_account.into_inner();
    let current_date = Utc::now().date_naive();
    let option_current_date: Option<NaiveDate> = Some(current_date);

    funcs::format_credentials(
        Some(&account.username), 
        Some(&account.password), 
        Some(&account.first_name), 
        Some(&account.last_name));

    let mut data = ResponseStruct { 
        response: String::new(),
        message: String::new() 
    };

    if user_exists(&account.username, None){
        data.response = String::from("BAD");
        data.message = funcs::ACCOUNT_CREATION_FAILURE_USER_EXISTS.to_string();
    } else {
        match insert_users(
            account.username, 
            account.password, 
            account.first_name, 
            account.last_name, 
            option_current_date) 
        {
            Ok(_) => {
                data.response = String::from("OK");
                data.message = funcs::ACCOUNT_CREATION_SUCCESS_MESSAGE.to_string();
        },
            Err(_) => {
                data.response = String::from("BAD");
                data.message = funcs::ACCOUNT_CREATION_FAILURE_MESSAGE.to_string();
            } 
        }    
        
    }  

    Json(data)

}

#[post("/login.html", data = "<request>")]
pub fn login(request: Json<LoginAccount>, cookies: &CookieJar<'_>) -> Redirect {

    let account = request.into_inner();

    funcs::format_credentials(
        Some(&account.username), 
        Some(&account.password),
        None,
        None);

    let mut data = ResponseStruct { 
        response: String::new(),
        message: String::new() 
    };
    
    if user_exists(&account.username, Some(&account.password)){
        data.message = funcs::ACCOUNT_LOGIN_SUCCESS_MESSAGE.to_string();
        data.response = String::from("OK");

        let user_id = get_user_id(&account.username).unwrap();
        let session_id = generate_session_id();

        match insert_session(&user_id, &session_id) {
            Ok(()) => {
                data.message = funcs::ACCOUNT_LOGIN_SUCCESS_MESSAGE.to_string();
                data.response = String::from("OK"); 
            }

            Err(e) => {
                println!("{:?}", e);
                data.message = funcs::ACCOUNT_LOGIN_FAILURE_SESSION_FAIL.to_string();
                data.response = String::from("BAD"); 
            }
        }
            

        let cookie_user_id = Cookie::new("user_id", user_id.to_string());
        let cookie_session_id = Cookie::new("session_id", session_id.to_string());

        cookies.add(cookie_user_id);
        cookies.add(cookie_session_id);

        Redirect::to("/home.html")
            
    } else {
        data.message = funcs::ACCOUNT_LOGIN_FAILURE_USER_NOT_FOUND_MESSAGE.to_string();
        data.response = String::from("BAD");

        Redirect::to("/")
    }


}

#[get("/")]
pub fn redirect_to_login() -> Redirect {
    let target_url = "/login.html";
    Redirect::to(target_url)
}

#[get("/test/<name>")]
pub fn test(name: &str) -> String {
    format!("You are {}!", name)
} 

#[get("/home.html")]
pub async fn get_home(cookies: &CookieJar<'_>) -> Result<NamedFile, rocket::http::Status> {

    let user_id_from_cookie = match funcs::get_user_id_from_cookie(cookies) {
        Ok(user_id) => user_id,
        Err(e) => {
            println!("{:?}", e);
            return Err(rocket::http::Status::Unauthorized);    
        }
    };

    let session_id_from_db = match get_session_id_from_db(&user_id_from_cookie.parse::<i32>().unwrap()) {
        Ok(id) => id,
        Err(e) => {
            println!("{:?}", e);
            return Err(rocket::http::Status::Unauthorized)
        }
    };
    
    let session_id_from_cookie = match funcs::get_session_id_from_cookie(cookies) {
        Ok(cookie) => cookie,
        Err(e) => {
            println!("{:?}", e);
            return Err(rocket::http::Status::Unauthorized);
        }
    };

    let file_path = Path::new(HOME_PATH);
    
    if session_id_from_db == session_id_from_cookie {
        NamedFile::open(file_path)
            .await
            .map_err(|_e| rocket::http::Status::NotFound)

    } else {
        Err(rocket::http::Status::Unauthorized)
    }

}

#[post("/home.html", data = "<request>")]
pub fn post_home(request: Json<StatusStruct>, cookies: &CookieJar<'_>) -> Json<> {
    let status = request.into_inner();

    if true {
        Ok(Redirect::to("/"))
    } else {
        Err(BadJSONData::StatusUnrecognized)
    }
}

