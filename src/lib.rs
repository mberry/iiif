use reqwest::{Client, Url};
use std::fs::File;
use std::io::prelude::*;
use bytes::Bytes;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Image {
  pub host: String,
  pub prefixes: Vec<String>,
  pub identifier: String,

  //Region THEN Size THEN Rotation THEN Quality THEN Format
  pub region: Region,
  pub size: Size,
  pub rotation: Rotation,
  pub quality: Quality,
  pub format: Format 
}

pub struct Response {
  pub status_code: u16,
  pub image: Bytes
}

impl Image {
  pub fn new(host: &str) -> Self {
    Image {
      host: host.into(),
      ..Default::default()
    }
  }

  fn build_parts(&self) -> Vec<String> {
    let mut out = self.prefixes.clone();
    let mut info = vec![
      self.identifier.clone(),
      self.region.to_string(),
      self.size.to_string(),
      self.rotation.to_string(),
      format!("{}.{}", self.quality.to_string(), self.format.to_string()),
    ];
    out.append(&mut info);
    out
  }

  fn build_info_parts(&self) -> Vec<String> {
    let mut out = self.prefixes.clone();
    let mut info = vec![self.identifier.to_string(), "info.json".to_string()];
    out.append(&mut info);
    out.to_vec()
  }

  fn build_uri(self, parts: Vec<String>) -> Url {
    let mut url = Url::parse(&self.host).expect("Parsing URL Host");

    for part in parts {
      url.path_segments_mut()
          .expect("Constructing URL Parts")
          .pop_if_empty()
          .push(&part);       
    }
    url
  }

  pub async fn request(self, client: &Client) -> Response {
    let parts = self.build_parts();
    let uri = self.build_uri(parts);
    let response = client.get(uri).send().await.unwrap();
    let status_code = response.status().as_u16();
    dbg!{&response};
    let image  = match status_code {
      200 => response.bytes().await.expect("Image payload"),
      _ => Bytes::new()
    };
    Response {
      status_code,
      image
    }
  }

  pub async fn info_request(self, client: &Client) -> InfoResponse {
    let parts = self.build_info_parts();
    let uri = self.build_uri(parts);
    let response = client.get(uri).send().await.unwrap();
    let status_code = response.status().as_u16();
    dbg!{&response};
    let info  = match status_code {
      200 => response.text().await.expect("Info payload"),
      _ => String::new()
    };
    InfoResponse {
      status_code,
      info
    }
  }
}

pub struct InfoResponse {
  status_code: u16,
  info: String
}

impl Response {
  pub async fn write_to_file(self, path: &str) -> std::io::Result<()>{
    let mut file = File::create(path).unwrap();
    file.write_all(&self.image)
  }
}

impl ToString for Region {
  fn to_string(&self) -> String {
    match self {
      Region::Full => "full".into(),
      Region::Square => "square".into(),
      Region::Abs(a) => a.to_string(),
      Region::Pct(p) => p.to_string()
    }
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Absolute {
  pub x: usize,
  pub y: usize,
  pub w: usize,
  pub h: usize,
}

impl ToString for Absolute {
  fn to_string(&self) -> String {
    join_coords(self.x, self.y, self.w, self.h)
  }
}

fn join_coords<T: ToString>(x: T, y: T, w: T, h: T) -> String {
  [x, y, w, h].iter()
  .map(|s| s.to_string())
  .collect::<Vec<String>>()
  .join(",")
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Percentage {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

impl ToString for Percentage {
  fn to_string(&self) -> String {
    let floats =format_floats(vec![self.x, self.y, self.w, self.h]);
    let coords = join_coords(&floats[0], &floats[1], &floats[2], &floats[3]);
    ["pct:".to_string(), coords].join("")
  }
}

// Limit float resolution down to 3 decimal places
fn format_floats<T: std::fmt::Display>(floats: Vec<T>) -> Vec<String> {
  floats.iter()
  .map(|float| format!("{:.5}", float.to_string()))
  .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Region {
  Full,
  Square,
  Abs(Absolute),
  Pct(Percentage)
}

impl ToString for Size {
  fn to_string(&self) -> String {
    match self {
      Size::Full => "full".into(),
      Size::Max => "max".into(),
      Size::W(w) => format!("{},", w),
      Size::H(h) => format!(",{}", h),
      Size::Pct(n) => format!("pct:{}", n),
      Size::WH(w,h) => format!("{},{}", w, h),
      Size::LtWH(w,h) => format!("!{},{}", w, h)
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
  Full,
  Max,
  W(usize),
  H(usize),
  Pct(u16),
  WH(usize,usize),
  LtWH(usize, usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
  N(f32),
  Mirroed(f32)
}

impl ToString for Rotation {
  // limit float values to 3 decimal places
  fn to_string(&self) -> String {
    match self {
      Rotation::N(n) => format!("{:.5}", n.to_string()),
      Rotation::Mirroed(n) =>  format!("!{:.5}", n.to_string())
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quality {
  ServerDefault,
  Color,
  Gray,
  Bitonal
}

impl ToString for Quality {
  fn to_string(&self) -> String {
    match self {
      Quality::ServerDefault => "default".into(),
      Quality::Color => "color".into(),
      Quality::Bitonal => "bitonal".into(),
      Quality::Gray => "gray".into()
    }
  }
}



//A format value that is unsupported should result in a 400 status code.
#[derive(Debug, Clone, PartialEq)]
pub enum Format {
  Jpg,
  Tif,
  Png,
  Gif,
  Jp2,
  Pdf,
  Webp
}

impl ToString for Format {
  fn to_string(&self) -> String {
    match self {
      Format::Jpg => "jpg".into(),
      Format::Tif => "tif".into(),
      Format::Png => "png".into(),
      Format::Gif => "gif".into(),
      Format::Jp2 => "jp2".into(),
      Format::Pdf => "pdf".into(),
      Format::Webp => "webp".into()
    }
  }
}

// Defaults image settings
impl Default for Region {
  fn default() -> Self { Region::Full }
}

impl Default for Size {
  fn default() -> Self { Size::Full }
}

impl Default for Rotation {
  fn default() -> Self { Rotation::N(0.0) }
}

impl Default for Quality {
  fn default() -> Self { Quality::ServerDefault }
}

impl Default for Format {
  fn default() -> Self { Format::Jpg }
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
