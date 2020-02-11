use bytes::Bytes;
use std::fs::File;
use std::io::prelude::*;

pub struct Response {
  pub status_code: u16,
  pub image: Bytes
}

impl Response {
  pub async fn write_to_file(self, path: &str) -> std::io::Result<()>{
    let mut file = File::create(path).unwrap();
    file.write_all(&self.image)
  }
}