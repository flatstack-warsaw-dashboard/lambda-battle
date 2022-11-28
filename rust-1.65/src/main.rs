use lambda_http::{run, http::StatusCode, service_fn, Error, IntoResponse, Request, Response, Body};
use lambda_http::http::HeaderValue;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

async fn function_handler(request: Request) -> Result<impl IntoResponse, Error> {
    if !check_content_type(&request) {
        return Ok(make_bad_request());
    }


    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(
                request
                    .body()
                    .clone()
            )
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
