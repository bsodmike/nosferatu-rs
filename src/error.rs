//! Error and Result module.

use crate::error_from;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::header::ToStrError;
use serde_json::json;
use std::error::Error as StdError;
use std::fmt;
use std::num::ParseIntError;
use std::{env::VarError, str::Utf8Error};
use tokio::task::JoinError;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    inner: BoxError,
}

impl Error {
    #[allow(dead_code)]
    /// Create a new `Error` from a boxable error.
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: error.into(),
        }
    }

    #[allow(dead_code)]
    /// Convert an `Error` back into the underlying boxed trait object.
    pub fn into_inner(self) -> BoxError {
        self.inner
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.inner)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self { inner: error }
    }
}

error_from!(anyhow::Error);
error_from!(VarError);
error_from!(serde_json::Error);
error_from!(ParseIntError);
error_from!(ToStrError);
error_from!(std::io::Error);
error_from!(std::string::FromUtf8Error);
error_from!(std::string::String);
error_from!(JoinError);
error_from!(Box<axum::http::Error>);
error_from!(regex::Error);
error_from!(uuid::Error);
error_from!(reqwest::Error);
error_from!(Utf8Error);
error_from!(hyper::header::InvalidHeaderValue);
error_from!(axum::http::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let _leak_error = false;
        let source = if let Some(ref source) = self.inner.source() {
            format!("{}", source)
        } else {
            String::default()
        };

        tracing::error!("Error: {}", source);

        let error_reason = "Internal Server Error".to_string();
        let body = Json(json!({
            "error": error_reason,
        }));

        let mut res = body.into_response();
        res.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/json"),
        );
        *res.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;

        res
    }
}

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! error_from {
        ($typ:ty) => {
            impl From<$typ> for Error {
                fn from(error: $typ) -> Self {
                    Self {
                        inner: error.into(),
                    }
                }
            }
        };
    }
}
