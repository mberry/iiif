use iiif::*;
// Writing an image to file.

// Example image
// https://ids.lib.harvard.edu/ids/iiif/25286607/full/500,/0/default.jpg
// Region: Full 
// Size: width 200px
// Rotation: None
// Default format and quality
// Written to foo.jpg

#[tokio::main]
async fn main() {
  // Basic Image API struct
  let mut api  = Image::new("https://ids.lib.harvard.edu");
  
  // Create a common client to reuse requests with
  let client = Client::new();
  
  // Set image info, directly modifying the api struct rather than using the 
  // helper methods.
  // Vector of optional prefixes
  // You can either set the prefix/es in the server field or here
  api.prefixes = vec!["ids".into(), "iiif".into()];
  
  // Image identifier
  api.identifier = "25286607".into();
  
  // Set region
  api.region = Region::Full;
  
  // Set size: width 500 pixels
  api.size = Size::W(200);

  // Make request
  let response = api.request(&client).await.expect("Requesting image");

  // Save image to file foo.jpg
  response.write_to_file("foo.jpg")
          .await
          .expect("Writing file to disk");
} 