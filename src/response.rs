use bytes::Bytes;
use std::fs::File;
use std::io::prelude::*;

pub struct Response {
  pub status_code: u16,
  pub image: Bytes
}

/// Contains a json string of image metadata and status code
pub struct InfoResponse {
  pub status_code: u16,
  pub info: String
}

impl Response {
  /// Writes the image response to a file at the specified relative or absolute path
  /// 
  ///  # Example
  /// ```
  /// let client = Client::new();
  /// let api = Image::new("https://ids.lib.harvard.edu/ids/iiif/");
  /// api.identifier = "25286607".into();
  /// api.request(&client)
  ///     .await
  ///     .write_to_file("foo.jpg")
  ///     .await
  ///     .expect("Writing file to disk");
  /// ```
  pub async fn write_to_file(self, path: &str) -> std::io::Result<()>{
    let mut file = File::create(path).unwrap();
    file.write_all(&self.image)
  }

  /// Returns true if the status code is in the range of 200-299, otherwise false
  pub fn success(&self) -> bool {
    match self.status_code {
      200..=299 => true,
      _ => false
    }
  }
}