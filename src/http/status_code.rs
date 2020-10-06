use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    BadRequest = 400,
    InternalServerError = 500,
    Unauthorized = 401,
    Forbidden = 403,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not found",
            Self::BadRequest => "Bad request",
            Self::Forbidden => "Forbidden",
            Self::InternalServerError => "Internal server error",
            Self::Unauthorized => "Unauthorized",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
