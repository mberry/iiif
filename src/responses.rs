//! Contains the response structs and their helper methods
use super::*;
use bytes::Bytes;
use std::fs::File;
use std::io::prelude::*;

/// Generic response struct containing a byte array of the image
/// the final url and status code returned 
#[derive(Clone, Debug)]
pub struct Response {
  pub status_code: u16,
  pub url: Url,
  pub image: Bytes
}

/// Contains the deserialized json info, the raw json string,
/// the url and status code
#[derive(Clone, Debug)]
pub struct InfoResponse {
  pub status_code: u16,
  pub raw_json: String,
  pub info: Info,
  pub url: Url
}

impl Response {
  /// Writes the image response to a file at the specified relative or absolute path
  /// 
  ///  # Example
  /// ```rust,ignore
  /// use iiif::*;
  /// let client = Client::new();
  /// let mut api = Image::new("https://ids.lib.harvard.edu/ids/iiif/");
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
}

impl InfoResponse {
  
  /// Returns a  set of height and width pairs the client should use in the size parameter to request complete images at different sizes that the server has available. This may be used to let a client know the sizes that are available when the server does not support requests for arbitrary sizes, or simply as a hint that requesting an image of this size may result in a faster response. A request constructed with the w,h syntax using these sizes must be supported by the server, even if arbitrary width and height are not.
  pub fn sizes(&self) -> Vec<InfoSize> {
    self.info.sizes.clone()
  }

  /// The width in pixels of the full image content, given as an integer.
  pub fn width(&self) -> usize {
    self.info.width
  }

  /// The height in pixels of the full image content, given as an integer.
  pub fn height(&self) -> usize {
    self.info.height
  }

  /// A set of descriptions of the parameters to use to request regions of the image (tiles) that are efficient for the server to deliver. Each description gives a width, optionally a height for non-square tiles, and a set of scale factors at which tiles of those dimensions are available.
  pub fn tiles(&self) -> Vec<Tile> {
    self.info.tiles.clone()
  }

  ///  Might include copyright or ownership statements, or a simple acknowledgement of the providing institution
  pub fn attribution(&self) -> Vec<Attribution> {
    self.info.attribution.clone()
  }


  pub fn license(&self) -> Vec<String> {
    self.info.license.clone()
  }

  /// A link to an external resource that describes the license or rights statement under which content obtained from the Image API service may be used.
  pub fn formats(&self) -> Vec<String> {
    self.info.profile.1.formats.clone()
  }

  /// The set of image quality parameter values available for the image. If not specified then clients should assume only qualities declared in the compliance level document.
  pub fn qualities(&self) -> Vec<String> {
    self.info.profile.1.qualities.clone()
  }

  /// The set of features supported for the image. If not specified then clients should assume only features declared in the compliance level document.
  pub fn supports(&self) -> Vec<String> {
    self.info.profile.1.supports.clone()
  }
}

// Structs for deserialization

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
  #[serde(default)]
  #[serde(rename = "@context")]
  pub context: String,
  #[serde(rename = "@id")]
  pub id: String,
  pub protocol: String,
  pub width: usize,
  pub height: usize,
  #[serde(default)]
  pub sizes: Vec<InfoSize>,
  #[serde(default)]
  pub tiles: Vec<Tile>,
  #[serde(default)]
  pub attribution: Vec<Attribution>,
  #[serde(default)]
  pub logo: Logo,
  #[serde(default)]
  pub license: Vec<String>,
  #[serde(default)]
  pub profile: (String, Profile),
  #[serde(default)]
  pub service: Vec<Service2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoSize {
  pub width: usize,
  pub height: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
  pub width: usize,
  #[serde(rename = "scaleFactors")]
  pub scale_factors: Vec<usize>,
  #[serde(default)]
  pub height: usize,
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
  pub profile: String,
  #[serde(rename = "physicalScale")]
  pub physical_scale: f64,
  #[serde(rename = "physicalUnits")]
  pub physical_units: String,
  #[serde(rename = "@id")]
  pub id: String,
}