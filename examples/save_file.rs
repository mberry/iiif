use iiif::*;

// Writing an image to file.

// Example image
//https://ids.lib.harvard.edu/ids/iiif/25286607/full/500,/0/default.jpg


#[tokio::main]
async fn main() {
  // Basic Image API struct
  let mut api  = Image::new("https://ids.lib.harvard.edu");
  
  // Create a common client to reuse requests with
  let client = Client::new();
  
  // Set image info
  // Vector of optional prefixes
  // You can either set the prefix/es in the server field or here
  api.prefixes = vec!("ids".into(), "iiif".into());
  
  // Image identifier
  api.identifier = "25286607".into();
  
  // Set region
  api.region = Region::Full;
  
  // Set size: width 500 pixels
  api.size = Size::W(500);

  // Rotation default: Rotation::N(0.0)

  // Quality and Format: default.jpg
  
  // Make request
  let response = api.request(&client).await;

  // Save image to file foo.jpg
  response.write_to_file("foo.jpg").await.expect("Writing file to disk");
} 