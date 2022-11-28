mod iteration;

use lambda_http::{run, http::StatusCode, service_fn, Error, IntoResponse, Request, Response, Body};
use lambda_http::http::HeaderValue;
use crate::iteration::Iteration;


#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

async fn function_handler(request: Request) -> Result<impl IntoResponse, Error> {
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

    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::Text(body_text))
            .unwrap()
    )
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
