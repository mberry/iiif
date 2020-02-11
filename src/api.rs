use super::*;
use parameters::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Image {
  pub host: String,
  pub prefixes: Vec<String>,
  pub identifier: String,
  pub region: Region,
  pub size: Size,
  pub rotation: Rotation,
  pub quality: Quality,
  pub format: Format 
}

impl Image {
  pub fn new(host: &str) -> Self {
    Image {
      host: host.into(),
      ..Default::default()
    }
  }
}