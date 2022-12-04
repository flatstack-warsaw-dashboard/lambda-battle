mod iteration;
mod store;
mod validate_request;

use std::fmt::Display;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, http::StatusCode, service_fn, Error, IntoResponse, Request, Response, Body};
use lambda_http::http::HeaderValue;
use crate::iteration::Iteration;
use crate::store::{add_iteration, find_iteration};
use crate::validate_request::validate_request;

use serde::Serialize;
use serde_json::json;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    run(service_fn(|request| function_handler(request, &client))).await
}

macro_rules! to_bad_response {
    ($lit:ident, $expr:expr) => {
        if $expr.is_err() {
           let error = $expr.err().unwrap();

           return Ok(error.to_bad_response());
        }

      let $lit = $expr.unwrap();
    };
    ($expr:expr) => {
        if $expr.is_err() {
           let error = $expr.err().unwrap();

           return Ok(error.to_bad_response());
        }
    }
}

#[derive(Debug)]
struct BadIterationParsing;

impl ToBadResponse for BadIterationParsing {
    fn to_bad_response(&self) -> Response<Body> {
        make_bad_response()
    }
}

fn format_body(err: &impl Display) -> Body {
    Body::Text(json!({
                    "error_message": err.to_string()
                }).to_string())
}

fn build_response(status_code: &StatusCode, err: &impl Display) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(format_body(err))
        .unwrap()
}

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

async fn function_handler(request: Request, client: &Client) -> Result<impl IntoResponse, Error> {
    bad_response!(validate_request(&request), StatusCode::BAD_REQUEST);

    to_bad_response!(body_text, get_body_text(&request));
    to_bad_response!(iteration, Iteration::try_from(&body_text).map_err(|_| BadIterationParsing));

    let added_item = add_iteration(client, &iteration).await;
    if added_item.is_err() {
        return Ok(make_bad_response());
    }

    let prev_item = find_iteration(client, &iteration).await;
    if prev_item.is_err() {
        return Ok(make_bad_response());
    }

    match prev_item.unwrap() {
        None => Ok(make_success_response(iteration).unwrap()),
        Some(iteration) => Ok(make_success_response(iteration).unwrap())
    }
}

fn make_success_response(body: impl Serialize) -> Result<Response<Body>, serde_json::Error> {
    serde_json::to_string(&body)
        .map(|serialized| {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::Text(serialized))
                .unwrap()
        })
}

trait ToBadResponse {
    fn to_bad_response(&self) -> Response<Body>;
}

#[derive(Debug)]
enum CheckContentTypeErrors {
    MissingContentType,
    ContentTypeNotSupported,
}

impl ToBadResponse for CheckContentTypeErrors {
    fn to_bad_response(&self) -> Response<Body> {
        match self {
            CheckContentTypeErrors::MissingContentType => make_bad_response(),
            CheckContentTypeErrors::ContentTypeNotSupported => make_bad_response(),
        }
    }
}

fn check_content_type(request: &Request) -> Result<bool, CheckContentTypeErrors> {
    let value = request
        .headers()
        .get("Content-Type");

    if value.is_none() {
        return Err(CheckContentTypeErrors::MissingContentType);
    }

    if !is_application_json(value.unwrap()) {
        return Err(CheckContentTypeErrors::ContentTypeNotSupported);
    }

    Ok(true)
}

fn is_application_json(header: &HeaderValue) -> bool {
    header.as_bytes() == "application/json".as_bytes()
}

fn make_bad_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::Empty)
        .unwrap()
}

#[derive(Debug)]
struct GetBodyTextError;

impl ToBadResponse for GetBodyTextError {
    fn to_bad_response(&self) -> Response<Body> {
        make_bad_response()
    }
}

fn get_body_text(request: &Request) -> Result<String, GetBodyTextError> {
    match request.body() {
        Body::Text(body) => Ok(body.clone()),
        _ => Err(GetBodyTextError)
    }
}
