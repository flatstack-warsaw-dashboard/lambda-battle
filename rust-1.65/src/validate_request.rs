use std::fmt::{Display, Formatter, write};
use lambda_http::http::HeaderValue;
use lambda_http::Request;

pub enum ValidationError<'a> {
    MissingContentType,
    ContentTypeNotSupported(&'a HeaderValue),
}

impl<'a> Display for ValidationError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::ContentTypeNotSupported( header) => write!(f, " `{:?}` content type not supported", header.as_bytes()),
            ValidationError::MissingContentType => write!(f, "missing `Content-Type` header")
        }
    }
}



pub fn validate_request(request: &Request) -> Result<bool, ValidationError> {
    check_content_type(request)
}


fn check_content_type(request: &Request) -> Result<bool, ValidationError> {
    let value = request
        .headers()
        .get("Content-Type");

    if value.is_none() {
        return Err(ValidationError::MissingContentType);
    }

    if  value.unwrap().as_bytes() != "application/json".as_bytes() {
        return Err(ValidationError::ContentTypeNotSupported(value.unwrap()))
    }

    Ok(true)
}
