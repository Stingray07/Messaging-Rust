use crate::rocket_files::rocket_structs::{CreateAccount, LoginAccount, ResponseStruct, StatusStruct};
use crate::rocket_files::rocket_funcs as funcs;
use crate::rocket_files::enums::BadJSONData;
use chrono::{NaiveDate, Utc};
use rocket::serde::json::Json;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use message::*;

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

#[post("/home.html", data = "<request>")]
pub fn post_home(request: Json<StatusStruct>, cookies: &CookieJar<'_>) -> Json<ResponseStruct> {
    let mut response = ResponseStruct {
        response: String::from(""),
        message: String::from(""),
    };

    let status = request.into_inner();
    let session_id_from_cookie = match funcs::get_session_id_from_cookie(cookies) {
        Ok(session_id) => session_id,
        Err(e) => {
            println!("{:?}", e);
            response.response = "BAD".to_string();
            response.message = "SESSION NOT FOUND IN COOKIE".to_string();
            return Json(response)
        }
    };

    match delete_session_id(&session_id_from_cookie) {
        Ok(_) => {
            response.response = "GOOD".to_string();
            response.message = "SESSION SUCCESSFULLY DELETED".to_string();
            return Json(response)
        }

        Err(e) => {
            println!("{:?}", e);
            response.response = "BAD".to_string();
            response.message = "SESSION NOT DELETED".to_string();
            return Json(response)
        }   
    }
}
