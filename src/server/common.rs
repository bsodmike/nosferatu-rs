use crate::error::Error;
use nosferatu::prelude::axum_prelude::*;
use serde_json::json;
use serde_json::Value;
use std::fmt;

pub async fn handle_health_get() -> Result<Response, Error> {
    Ok(return_json(json!({ "status": "success" }), None)?.into_response())
}

pub fn return_json(json: Value, status: Option<StatusCode>) -> Result<Response<Body>, Error> {
    let status = status.unwrap_or(StatusCode::OK);

    let resp = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(json.to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}

pub struct NetworkAddr<'a>(&'a str, u16);

impl<'a> NetworkAddr<'a> {
    pub fn new(host: &'a str, port: u16) -> Self {
        Self(host, port)
    }

    pub fn host(&'a self) -> &'a str {
        self.0
    }

    pub fn port(&'a self) -> u16 {
        self.1
    }
}

impl<'a> fmt::Display for NetworkAddr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}
