mod iteration;
mod store;

use aws_sdk_dynamodb::Client;
use lambda_http::{run, http::StatusCode, service_fn, Error, IntoResponse, Request, Response, Body};
use lambda_http::http::HeaderValue;
use crate::iteration::Iteration;
use crate::store::{add_iteration, find_iteration};
use serde::Serialize;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    run(service_fn(|request| function_handler(request, &client))).await
}

async fn function_handler(request: Request, client: &Client) -> Result<impl IntoResponse, Error> {
    if !check_content_type(&request) {
        return Ok(make_bad_request());
    }

    let body_text_opt = get_body_text(&request);
    if body_text_opt.is_none() {
        return Ok(make_bad_request());
    }
    let body_text = body_text_opt.unwrap();

    let iteration_wrapped = Iteration::try_from(&body_text);
    if iteration_wrapped.is_err() {
        return Ok(make_bad_request());
    }
    let iteration = iteration_wrapped.unwrap();

    let added_item = add_iteration(client, &iteration).await;
    if added_item.is_err() {
        return Ok(make_bad_request());
    }

    let prev_item = find_iteration(client, &iteration).await;
    if prev_item.is_err() {
        return Ok(make_bad_request());
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

fn check_content_type(request: &Request) -> bool {
    request
        .headers()
        .get("Content-Type")
        .map_or(false, is_application_json)
}

fn is_application_json(header: &HeaderValue) -> bool {
    header.as_bytes() == "application/json".as_bytes()
}

fn make_bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::Empty)
        .unwrap()
}

fn get_body_text(request: &Request) -> Option<String> {
    match request.body() {
        Body::Text(body) => Some(body.clone()),
        _ => None
    }
}
