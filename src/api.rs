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
  /// Creates a new Image API 
  pub fn new(host: &str) -> Image {
    Image {
      host: host.into(),
      ..Default::default()
    }
  }

  pub fn identifier(&mut self, identifier: &str) {
    self.identifier = identifier.into()
  }

  /// Sets the current image region to full
  pub fn full_region(&mut self) {
    self.region = Region::Full;
  }

  pub fn square_region(&mut self) {
    self.region = Region::Square;
  }

  pub fn absolute_region(&mut self, x: usize, y: usize, w: usize, h: usize) {
    self.region = Region::Abs(Absolute{ x, y, w, h });
  }

  pub fn pct_region(&mut self,  x: f32, y: f32, w: f32, h: f32) {
    self.region = Region::Pct({Percentage{ x, y, w, h }})
  }

  pub fn max_size(&mut self) {
    self.size = Size::Max;
  }

  /// The image or region is scaled so that width is equal to w
  /// 
  /// Do not use this method in conjunction with height, use `width_height(w, h)` instead.
  pub fn width(&mut self, w: usize) {
    self.size = Size::W(w);
  }

  pub fn height(&mut self, w: usize) {
    self.size = Size::W(w);
  }

  pub fn pct_size(&mut self, n: u16) {
    self.size = Size::Pct(n);
  }

  pub fn width_height(&mut self, w: usize, h: usize) {
    self.size = Size::WH(w,h);
  }

  
  pub fn less_than_width_height(&mut self, w: usize, h: usize) {
    self.size = Size::LtWH(w,h);
  }

  pub fn mirrored(&mut self) {
    self.rotation = Rotation::Mirror(0.0);
  }

  pub fn rotate_right(&mut self) {
    self.rotation = Rotation::Normal(90.0);
  }


  pub fn rotate_left(&mut self) {
    self.rotation = Rotation::Normal(270.0);
  }

  pub fn full_color(&mut self) {
    self.quality = Quality::Color;
  }

  pub fn gray(&mut self) {
    self.quality = Quality::Gray;
  }

  pub fn bitonal(&mut self) {
    self.quality = Quality::Bitonal;
  }

  pub fn jpg(&mut self) {
    self.format = Format::Jpg;
  }

  pub fn tif(&mut self) {
    self.format = Format::Tif;
  }

  pub fn png(&mut self) {
    self.format = Format::Png;
  }

  pub fn gif(&mut self) {
    self.format = Format::Gif;
  }

  pub fn jp2(&mut self) {
    self.format = Format::Jp2;
  }

  pub fn pdf(&mut self) {
    self.format = Format::Pdf;
  }

  pub fn webp(&mut self) {
    self.format = Format::Webp;
  }

}