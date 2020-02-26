//! Server response errors
use std::error::Error;
use std::fmt;

/// Holds the status code and IIIF error description
#[derive(Debug)]
pub struct ResponseError {
    pub status_code: u16,
    pub details: String
}

impl ResponseError {
  pub fn new(status_code: u16) -> ResponseError {
    ResponseError{
      status_code,
      details: parse_status(status_code)
    }
  }
}

impl fmt::Display for ResponseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f,"{}: {}", self.status_code, self.details)
  }
}

impl Error for ResponseError  {
  fn description(&self) -> &str {
      &self.details
  }
}

fn parse_status(status_code: u16) -> String {
  match status_code {
    400 => "The server cannot fulfill the request, as the syntax of the request issued by the client is incorrect.".into(),
    401 => "Authentication is required and not provided. See https://iiif.io/api/image/2.1/#authentication".into(),
    403 => "The user, authenticated or not, is not permitted to perform the requested operation.".into(),
    404 => "The image resource specified by identifier does not exist, the value of one or more of the parameters is not supported for this image, or the requested size is greater than the limits specified.".into(),
    500 => "The server encountered an unexpected error that prevented it from fulfilling the request.".into(),
    501 => "The server received a valid IIIF request that is not implemented.".into(),
    503 => "The server is busy/temporarily unavailable due to load/maintenance issues.".into(),
    _ => "Unspecified Error, check status code".into()
  }
}