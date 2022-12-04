use std::fmt::{Display, Formatter};
use lambda_http::{Request, Body};

#[derive(Debug)]
pub struct GetBodyTextError;

impl Display for GetBodyTextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "request body should be string")
    }
}

pub fn get_body_string(request: &Request) -> Result<&String, GetBodyTextError> {
    match request.body() {
        Body::Text(value) => Ok(value),
        _ => Err(GetBodyTextError)
    }
}
