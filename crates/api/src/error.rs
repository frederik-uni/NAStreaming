use std::fmt::{Display, Formatter};

use actix_web::{http::StatusCode, ResponseError};
use apistos::ApiErrorComponent;

pub type StartUpResult<T> = Result<T, StartUpError>;
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, ApiErrorComponent)]
#[openapi_error(status(code = 405, description = "Invalid input"))]
pub enum ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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
pub enum StartUpError {
    DisplayConfig(toml::ser::Error),
    ParseConfig(toml::de::Error),
    CreateConfig(std::io::Error),
    ReadConfig(std::io::Error),
}
