use super::*;

/// The region parameter defines the rectangular portion of the 
/// full image to be returned. Region can be specified by pixel coordinates, 
/// percentage or by the value “full”, which specifies that the entire image 
/// should be returned.
/// 
/// The default is Region::Full.
/// 
/// | Form | Description |
/// |-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | Full | The complete image is returned, without any cropping. |
/// | Square | The region is defined as an area where the width and height are both equal to the length of the shorter dimension of the complete image. The region may be positioned anywhere in the longer dimension of the image content at the server’s discretion, and centered is often a reasonable default. |
/// | Absolute(x,y,w,h) | The region of the full image to be returned is specified in terms of absolute pixel values. The value of x represents the number of pixels from the 0 position on the horizontal axis. The value of y represents the number of pixels from the 0 position on the vertical axis. Thus the x,y position 0,0 is the upper left-most pixel of the image. w represents the width of the region and h represents the height of the region in pixels. |
/// | Percentage(x,y,w,h) | The region to be returned is specified as a sequence of percentages of the full image’s dimensions, as reported in the image information document. Thus, x represents the number of pixels from the 0 position on the horizontal axis, calculated as a percentage of the reported width. w represents the width of the region, also calculated as a percentage of the reported width. The same applies to y and h respectively. These may be floating point numbers. |#[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub enum Region {
  Full,
  Square,
  Abs(Absolute),
  Pct(Percentage)
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Absolute {
  pub x: usize,
  pub y: usize,
  pub w: usize,
  pub h: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Percentage {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

/// The size parameter determines the dimensions to which the extracted region is to be scaled.
/// 
/// The default is full, this setting will be deprecated in version 3.0 in favor of max
/// 
/// | Form | Description |
/// |-------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | Full | The image or region is not scaled, and is returned at its full size. Note deprecation warning. |
/// | Max | The image or region is returned at the maximum size available, as indicated by maxWidth, maxHeight, maxArea in the profile description. This is the same as full if none of these properties are provided. |
/// | W(w) | The image or region should be scaled so that its width is exactly equal to w, and the height will be a calculated value that maintains the aspect ratio of the extracted region. |
/// | H(h| The image or region should be scaled so that its height is exactly equal to h, and the width will be a calculated value that maintains the aspect ratio of the extracted region. |
/// | Pct(n) | The width and height of the returned image is scaled to n% of the width and height of the extracted region. The aspect ratio of the returned image is the same as that of the extracted region. |
/// | WH(w,h) | The width and height of the returned image are exactly w and h. The aspect ratio of the returned image may be different than the extracted region, resulting in a distorted image. |
/// | LtWH(w,h) | The image content is scaled for the best fit such that the resulting width and height are less than or equal to the requested width and height. The exact scaling may be determined by the service provider, based on characteristics including image quality and system performance. The dimensions of the returned image content are calculated to maintain the aspect ratio of the extracted region. |
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

/// The rotation parameter specifies mirroring and rotation. A leading exclamation mark (“!”) indicates that the image should be mirrored by reflection on the vertical axis before any rotation is applied. The numerical value represents the number of degrees of clockwise rotation, and may be any floating point number from 0 to 360.
/// 
/// The default is Rotation::N(0.0)
/// 
/// | Form | Description |
/// |------|---------------------------------------------------------|
/// | Normal(n) | The degrees of clockwise rotation from 0 up to 360. |
/// | Mirror(n) | The image should be mirrored and then rotated as above. |
#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
  Normal(f32),
  Mirror(f32)
}

/// The quality parameter determines whether the image is delivered in color, grayscale or black and white.
/// 
/// The default is Quality::ServerDefault
/// 
/// | Quality | Parameter Returned |
/// |---------|-------------------------------------------------------------------------------------------------------|
/// | color | The image is returned in full color. |
/// | gray | The image is returned in grayscale, where each pixel is black, white or any shade of gray in between. |
/// | bitonal | The image returned is bitonal, where each pixel is either black or white. |
/// | default | The image is returned using the server’s default quality (e.g. color, gray or bitonal) for the image. |
#[derive(Debug, Clone, PartialEq)]
pub enum Quality {
  ServerDefault,
  Color,
  Gray,
  Bitonal
}

///The format of the returned image is expressed as an extension at the end of the URI.
/// 
/// The default is Format::Jpg
/// 
/// A format value that is unsupported should result in a 400 status code.
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

impl ToString for Absolute {
  fn to_string(&self) -> String {
    join_coords(self.x, self.y, self.w, self.h)
  }
}

impl ToString for Percentage {
  fn to_string(&self) -> String {
    let floats =format_floats(vec![self.x, self.y, self.w, self.h]);
    let coords = join_coords(&floats[0], &floats[1], &floats[2], &floats[3]);
    ["pct:".to_string(), coords].join("")
  }
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

impl ToString for Rotation {
  // limit float values to 3 decimal places
  fn to_string(&self) -> String {
    match self {
      Rotation::Normal(n) => format!("{:.5}", n.to_string()),
      Rotation::Mirror(n) =>  format!("!{:.5}", n.to_string())
    }
  }
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
  fn default() -> Self { Rotation::Normal(0.0) }
}

impl Default for Quality {
  fn default() -> Self { Quality::ServerDefault }
}

impl Default for Format {
  fn default() -> Self { Format::Jpg }
}