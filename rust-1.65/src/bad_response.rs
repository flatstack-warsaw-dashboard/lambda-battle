use serde_json::json;
use lambda_http::{Response, Body, http::StatusCode};
use std::fmt::Display;

fn format_body(err: &impl Display) -> Body {
    Body::Text(json!({
                    "error_message": err.to_string()
                }).to_string())
}

pub fn build_response(status_code: &StatusCode, err: &impl Display) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(format_body(err))
        .unwrap()
}

#[macro_export]
macro_rules! bad_response {
    ($expr:expr, $status_code:expr) => {
        if $expr.is_err() {
            let error = $expr.err().unwrap();
            return Ok(build_response(&$status_code, &error));
        }
    };
      ($lit:ident, $expr:expr, $status_code:expr) => {
        if $expr.is_err() {
            let error = $expr.err().unwrap();
            return Ok(build_response(&$status_code, &error));
        }
        let $lit = $expr.unwrap();
    };
}
