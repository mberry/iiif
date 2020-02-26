//! The main Image API 
use crate::parameters::*;

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
  /// Creates a new Image API with the host required or optionally the host 
  /// + prefixes. The host argument requires the full base url with the scheme 
  /// included. 
  ///
  /// ```rust,no_test
  /// use iiif::*;
  /// 
  /// // Including prefixes upfront
  /// let api1 = Image::new(https://ids.lib.harvard.edu/ids/iiif/);
  /// 
  /// // Adding prefixes later
  /// let mut api2 = Image::new("https://ids.lib.harvard.edu");
  /// api2.prefixes(vec!["ids", "iiif"]);
  /// ```
  /// 
  pub fn new(host: &str) -> Image {
    Image {
      host: host.into(),
      ..Default::default()
    }
  }

  /// Sets the prefix/es, these can be optionally provided in the host field 
  /// when creating a new Image struct, if placed in there they can't be changed later. 
  pub fn prefixes(&mut self, prefixes: Vec<&str>) {
    self.prefixes = prefixes.iter().map(|s| s.to_string()).collect();
  }

  /// Set the image identifier for the next request
  pub fn identifier(&mut self, identifier: &str) {
    self.identifier = identifier.into()
  }

  /// Sets the current image region to full, this is the default and unnecessary 
  /// for a newly created Image struct
  pub fn full_region(&mut self) {
    self.region = Region::Full;
  }

  /// Sets the region is defined as an area where the width and height are both equal to the length of the shorter dimension of the complete image. The region may be positioned anywhere in the longer dimension of the image content at the server’s discretion, and centered is often a reasonable default.
  pub fn square_region(&mut self) {
    self.region = Region::Square;
  }

  /// Sets the region of the full image to be returned is specified in terms of absolute pixel values. The value of x represents the number of pixels from the 0 position on the horizontal axis. The value of y represents the number of pixels from the 0 position on the vertical axis. Thus the x,y position 0,0 is the upper left-most pixel of the image. w represents the width of the region and h represents the height of the region in pixels.
  pub fn absolute_region(&mut self, x: usize, y: usize, w: usize, h: usize) {
    self.region = Region::Abs(Absolute{ x, y, w, h });
  }

  /// Sets region to be returned is specified as a sequence of percentages of the full image’s dimensions, as reported in the image information document. Thus, x represents the number of pixels from the 0 position on the horizontal axis, calculated as a percentage of the reported width. w represents the width of the region, also calculated as a percentage of the reported width. The same applies to y and h respectively
  pub fn pct_region(&mut self,  x: f32, y: f32, w: f32, h: f32) {
    self.region = Region::Pct({Percentage{ x, y, w, h }})
  }

  /// Sets the  image or region is returned at the maximum size available, as indicated by maxWidth, maxHeight, maxArea in the profile description.
  pub fn max_size(&mut self) {
    self.size = Size::Max;
  }

  /// Sets the  image or region should be scaled so that its width is exactly equal to w, and the height will be a calculated value that maintains the aspect ratio of the extracted region.
  /// 
  /// Do not use this method in conjunction with height, use `width_height(w, h)` instead.
  pub fn width(&mut self, w: usize) {
    self.size = Size::W(w);
  }

  /// Sets the image or region should be scaled so that its height is exactly equal to h, and the width will be a calculated value that maintains the aspect ratio of the extracted region.
  pub fn height(&mut self, w: usize) {
    self.size = Size::W(w);
  }

  /// Sets the width and height of the returned image is scaled to n% of the width and height of the extracted region. The aspect ratio of the returned image is the same as that of the extracted region.
  pub fn pct_size(&mut self, n: u16) {
    self.size = Size::Pct(n);
  }

  /// Sets the  width and height of the returned image are exactly w and h. The aspect ratio of the returned image may be different than the extracted region, resulting in a distorted image.
  pub fn width_height(&mut self, w: usize, h: usize) {
    self.size = Size::WH(w,h);
  }

  /// Sets the  image content is scaled for the best fit such that the resulting width and height are less than or equal to the requested width and height. The exact scaling may be determined by the service provider, based on characteristics including image quality and system performance. The dimensions of the returned image content are calculated to maintain the aspect ratio of the extracted region.
  pub fn less_than_width_height(&mut self, w: usize, h: usize) {
    self.size = Size::LtWH(w,h);
  }

  /// Sets the image to be mirrored
  pub fn mirrored(&mut self) {
    self.rotation = Rotation::Mirror(0.0);
  }

  /// Sets the image to be rotated 90 degrees
  pub fn rotate_right(&mut self) {
    self.rotation = Rotation::Normal(90.0);
  }

  /// Sets the image to be rotated 270 degrees
  pub fn rotate_left(&mut self) {
    self.rotation = Rotation::Normal(270.0);
  }

  /// Sets the quality to be full color
  pub fn full_color(&mut self) {
    self.quality = Quality::Color;
  }

  /// Sets the image quality to be gray
  pub fn gray(&mut self) {
    self.quality = Quality::Gray;
  }

  /// Sets the image quality to be bitonal
  pub fn bitonal(&mut self) {
    self.quality = Quality::Bitonal;
  }

  /// Sets the image format to be jpg 
  pub fn jpg(&mut self) {
    self.format = Format::Jpg;
  }

  /// Sets the image format to be tif
  pub fn tif(&mut self) {
    self.format = Format::Tif;
  }

  /// Sets the image format to be png
  pub fn png(&mut self) {
    self.format = Format::Png;
  }

  /// Sets the image format to be  gif
  pub fn gif(&mut self) {
    self.format = Format::Gif;
  }

  /// Sets the image format to be  jp2
  pub fn jp2(&mut self) {
    self.format = Format::Jp2;
  }

  /// Sets the image format to be  pdf
  pub fn pdf(&mut self) {
    self.format = Format::Pdf;
  }

  /// Sets the image format to be webp
  pub fn webp(&mut self) {
    self.format = Format::Webp;
  }
}