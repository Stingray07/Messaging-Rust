use std::fmt::{self};
use crate::rocket_funcs::{SESSION_ID_NOT_FOUND_COOKIE,  USER_ID_NOT_FOUND_COOKIE};

const STATUS_UNRECOGNIZED: &str = "STATUS UNRECOGNIZED: Inavalid status report";

#[derive(Debug)]
pub enum NotFoundError {
    SessionIdNotFoundFromCookie,
    UserIdNotFoundFromCookie
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotFoundError::SessionIdNotFoundFromCookie => write!(f,"{:?}", SESSION_ID_NOT_FOUND_COOKIE),
            NotFoundError::UserIdNotFoundFromCookie => write!(f, "{:?}", USER_ID_NOT_FOUND_COOKIE)
        }
    }
}

pub enum BadJSONData {
    StatusUnrecognized
}

impl fmt::Display for BadJSONData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BadJSONData::StatusUnrecognized => write!(f, "{:?}", STATUS_UNRECOGNIZED)
        }
    }
}