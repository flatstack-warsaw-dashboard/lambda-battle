mod iteration;
mod store;
mod validate_request;
mod body_string;
#[macro_use]
mod bad_response;

use aws_sdk_dynamodb::Client;
use lambda_http::{run, http::StatusCode, service_fn, Error, IntoResponse, Request, Response, Body};
use crate::iteration::Iteration;
use crate::store::{add_iteration, find_iteration};
use crate::validate_request::validate_request;
use crate::body_string::get_body_string;
use serde::Serialize;

use crate::bad_response::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    run(service_fn(|request| function_handler(request, &client))).await
}

async fn function_handler(request: Request, client: &Client) -> Result<impl IntoResponse, Error> {
    bad_response!(validate_request(&request), StatusCode::BAD_REQUEST);
    bad_response!(body_string, get_body_string(&request), StatusCode::BAD_REQUEST);
    bad_response!(iteration, Iteration::try_from(body_string), StatusCode::BAD_REQUEST);
    bad_response!(add_iteration(client, &iteration).await, StatusCode::BAD_REQUEST);
    bad_response!(prev_item, find_iteration(client, &iteration).await, StatusCode::BAD_REQUEST);

    match prev_item {
        None => Ok(make_success_response(&iteration)),
        Some(iteration) => Ok(make_success_response(&iteration))
    }
}

fn make_success_response(body: &impl Serialize) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text(serde_json::to_string(body).unwrap()))
        .unwrap()
}
