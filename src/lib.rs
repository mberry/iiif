mod url;
mod utils;
mod api;
mod request;
mod response;
mod parameters;

use utils::*;
use bytes::Bytes;
pub use api::*;
pub use parameters::*;
pub use reqwest::{Client, Url};


pub struct InfoResponse {
  pub status_code: u16,
  pub info: String
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn absolute_to_string() {
    let abs = Absolute {x: 1,y: 2,w: 3,h: 4};
    assert_eq!("1,2,3,4", abs.to_string());
  }
  #[test]
  fn percentage_to_string() {
    let pct = Percentage {x: 1.2345,y: 2f32,w: 3.03,h: 4.0};
    assert_eq!("pct:1.234,2,3.03,4", pct.to_string());
  }
}
