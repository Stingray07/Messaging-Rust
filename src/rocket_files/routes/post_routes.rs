use crate::rocket_files::rocket_structs::{CreateAccount, LoginAccount, ResponseStruct, StatusStruct, SharedData};
use crate::rocket_files::rocket_funcs as funcs;
use rocket::State;
use rocket::serde::json::Json;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use crate::diesel_funcs as diesel_funcs;
use crate::consts as message;


#[post("/create_account.html", data = "<new_account>")]
pub fn create_account(new_account: Json<CreateAccount>) -> Json<ResponseStruct> {
    let mut account = new_account.into_inner();

    funcs::format_credentials(
        Some(&account.username), 
        Some(&account.password), 
        Some(&account.first_name), 
        Some(&account.last_name));

    let mut data = funcs::create_response_struct();

    if diesel_funcs::user_exists(&account.username, None){
        funcs::modify_response_stuct(&mut data,
            String::from("BAD"),
            message::ACCOUNT_CREATION_FAILURE_USER_EXISTS_MESSAGE.to_string());

    } else {
        match diesel_funcs::insert_users(&mut account) 
        {
            Ok(_) => {
                funcs::modify_response_stuct(&mut data, 
                    String::from("OK"), 
                    message::ACCOUNT_CREATION_SUCCESS_MESSAGE.to_string());
        },
            Err(_) => {
                funcs::modify_response_stuct(&mut data, 
                    String::from("BAD"), 
                    message::ACCOUNT_CREATION_FAILURE_MESSAGE.to_string());
            } 
        }         
    }  

    Json(data)

}

#[post("/login.html", data = "<request>")]
pub fn login(request: Json<LoginAccount>, cookies: &CookieJar<'_>, shared_data: &State<SharedData>) -> Redirect {
    println!("VALUE FROM POST_LOGIN = {:?}", shared_data.value);
    let account = request.into_inner();

    funcs::format_credentials(
        Some(&account.username), 
        Some(&account.password),
        None,
        None);
    
    if diesel_funcs::user_exists(&account.username, Some(&account.password)){

        let user_id = diesel_funcs::get_user_id(&account.username).unwrap();
        let session_id = diesel_funcs::generate_session_id();

        match diesel_funcs::insert_session(&user_id, &session_id) {
            Ok(()) => {
                println!("{:?}", message::SESSION_ID_INSERTION_SUCCESSFUL_MESSAGE);
            }

            Err(e) => {
                println!("{:?}", message::SESSION_ID_INSERTION_FAILURE_MESSAGE);
                println!("{:?}", e);
            }
        }

        let cookie_username = Cookie::new("username", account.username);
        let cookie_user_id = Cookie::new("user_id", user_id.to_string());
        let cookie_session_id = Cookie::new("session_id", session_id.to_string());

        cookies.add(cookie_user_id);
        cookies.add(cookie_session_id);
        cookies.add(cookie_username);

        Redirect::to("/home.html")
            
    } else {
        println!("{:?}", message::ACCOUNT_LOGIN_FAILURE_USER_NOT_FOUND_MESSAGE);
        Redirect::to("/")
    }
}

#[post("/home.html", data = "<request>")]
pub fn post_home(request: Json<StatusStruct>, cookies: &CookieJar<'_>) -> Result<Json<ResponseStruct>, Redirect> {
    let mut response = funcs::create_response_struct();

    let status = request.into_inner();
    let session_id_from_cookie = match funcs::get_session_id_from_cookie(cookies) {
        Ok(session_id) => session_id,
        Err(e) => {
            println!("{:?}", e);
            funcs::modify_response_stuct(&mut response,
                String::from("BAD"),
                message::SESSION_ID_NOT_FOUND_COOKIE.to_string());
            return Ok(Json(response))
        }
    };

    match diesel_funcs::delete_session_id(&session_id_from_cookie) {
        Ok(_) => {
            funcs::modify_response_stuct(&mut response,
                String::from("GOOD"),
                message::SESSION_ID_DELETION_SUCCESSFUL_MESSAGE.to_string());
            return Err(Redirect::to("/"))
        }

        Err(e) => {
            println!("{:?}", e);
            funcs::modify_response_stuct(&mut response,
                String::from("BAD"),
                message::SESSION_ID_DELETION_UNSUCCESSFUL_MESSAGE.to_string());
            return Ok(Json(response))
        }   
    }
}
