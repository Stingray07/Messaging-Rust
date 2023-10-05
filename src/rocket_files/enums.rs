use std::fmt::{self};
use crate::consts::{SESSION_ID_NOT_FOUND_COOKIE, USER_ID_NOT_FOUND_COOKIE, USERNAME_NOT_FOUND_COOKIE}; 

#[derive(Debug)]
pub enum NotFoundError {
    SessionIdNotFoundFromCookie,
    UserIdNotFoundFromCookie,
    UsernameNotFoundFromCookie
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotFoundError::SessionIdNotFoundFromCookie => write!(f,"{:?}", SESSION_ID_NOT_FOUND_COOKIE),
            NotFoundError::UserIdNotFoundFromCookie => write!(f, "{:?}", USER_ID_NOT_FOUND_COOKIE),
            NotFoundError::UsernameNotFoundFromCookie => write!(f, "{:?}", USERNAME_NOT_FOUND_COOKIE)
        }
    }
}
