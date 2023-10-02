use crate::rocket_files::rocket_funcs as funcs;
use rocket::response::Redirect;
use rocket::http:: CookieJar;
use message::*;
use rocket::fs::NamedFile;
use std::path::Path;

const HOME_PATH: &str = "static/home.html";

#[get("/")]
pub fn redirect_to_login() -> Redirect {
    let target_url = "/login.html";
    Redirect::to(target_url)
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