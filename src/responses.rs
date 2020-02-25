use bytes::Bytes;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

/// Generic response struct
#[derive(Clone, Debug)]
pub struct Response {
  pub status_code: u16,
  pub image: Bytes
}

/// Contains a json string of image metadata and status code
#[derive(Clone, Debug, Default, Deserialize)]
pub struct InfoResponse {
  pub status_code: u16,
  pub info: Info
}

impl Response {
  /// Writes the image response to a file at the specified relative or absolute path
  /// 
  ///  # Example
  /// ```rust,no_run
  /// #[tokio::main]
  /// async fn main() {
  ///   use iiif::*;
  ///   let client = Client::new();
  ///   let mut api = Image::new("https://ids.lib.harvard.edu/ids/iiif/");
  ///   api.identifier = "25286607".into();
  ///   api.request(&client)
  ///       .await
  ///       .write_to_file("foo.jpg")
  ///       .await
  ///       .expect("Writing file to disk");
  /// }
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

// impl InfoResponse {
//   fn
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub protocol: String,
    pub width: i64,
    pub height: i64,
    pub sizes: Vec<InfoSize>,
    pub tiles: Vec<Tile>,
    pub attribution: Option<Vec<Attribution>>,
    pub logo: Option<Logo>,
    pub license: Vec<String>,
    pub profile: (String, Profile),
    pub service: Vec<Service2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoSize {
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub width: i64,
    #[serde(rename = "scaleFactors")]
    pub scale_factors: Vec<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribution {
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@language")]
    pub language: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logo {
    #[serde(rename = "@id")]
    pub id: String,
    pub service: Service,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub profile: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub formats: Vec<String>,
    pub qualities: Vec<String>,
    pub supports: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service2 {
    #[serde(rename = "@context")]
    pub context: String,
    pub profile: Option<String>,
    #[serde(rename = "physicalScale")]
    pub physical_scale: Option<f64>,
    #[serde(rename = "physicalUnits")]
    pub physical_units: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
}