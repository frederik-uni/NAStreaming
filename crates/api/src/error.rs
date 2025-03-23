use std::fmt::{Display, Formatter};

use actix_web::{http::StatusCode, ResponseError};
use apistos::ApiErrorComponent;

pub type StartUpResult<T> = Result<T, StartUpError>;
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, ApiErrorComponent)]
#[openapi_error(status(code = 405, description = "Invalid input"))]
pub enum ApiError {
    NotImplemented,
    Auth(jsonwebtoken::errors::Error),
    GenerateJwt(jsonwebtoken::errors::Error),
    ExpiredToken,
    NotFoundInDb,
    NoPermission,
    Conflict(String),
    Bcrypt(bcrypt::BcryptError),
    Surreal(models::Error),
    LoginFailed,
    InvalidBirthdate(chrono::ParseError),
}

impl From<chrono::ParseError> for ApiError {
    fn from(error: chrono::ParseError) -> Self {
        Self::InvalidBirthdate(error)
    }
}

impl From<models::Error> for ApiError {
    fn from(error: models::Error) -> Self {
        Self::Surreal(error)
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(error: bcrypt::BcryptError) -> Self {
        Self::Bcrypt(error)
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::Auth(error)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum StartUpError {
    DisplayConfig(toml::ser::Error),
    ParseConfig(toml::de::Error),
    CreateConfig(std::io::Error),
    ReadConfig(std::io::Error),
}
