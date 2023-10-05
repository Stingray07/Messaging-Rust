use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginAccount {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct CreateAccount{
    pub username: String, 
    pub password: String,
    pub first_name: String, 
    pub last_name: String
}

#[derive(Serialize)]
pub struct ResponseStruct {
    pub response: String,
    pub message: String
}

#[derive(Deserialize)]
pub struct StatusStruct{
    pub status: String,
}

pub struct SharedData {
    pub value : i32,
}