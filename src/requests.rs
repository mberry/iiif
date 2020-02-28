//! Contains the various request functions for the Image struct
use super::*;
use std::error::Error;
use crate::errors::*;


impl Image {
  /// Makes an asynchronous request with the current parameters using a reusable 
  /// http client and returns a response struct containing a byte array of the 
  /// image. This method is recommended for anything beyond making a few 
  /// requests as it can take advantage of keep-alive pooling.
  pub async fn request(self, client: &Client) -> Result<Response, Box<dyn Error>> {
    let parts = self.build_parts();
    let url = self.build_uri(parts);
    let response = client.get(url.clone())
                          .send()
                          .await?;
    let status_code = response.status().as_u16();
    let image = response.bytes().await?;
    match status_code {
      200..=299 =>  Ok(Response{status_code, url, image}),
      _ => Err(Box::new(ResponseError::new(status_code)))
    }
  }

  /// Makes an asynchronous request with the current parameters using a reusable 
  /// http client and returns both the raw json string along with the deserialized 
  /// InfoResponse struct which has numerous helper methods.
  /// This function is recommended for anything beyond making a few 
  /// requests as it can take advantage of keep-alive pooling.
  pub async fn request_info(self, client: &Client) -> Result<InfoResponse, Box<dyn Error>> {
    let parts = self.build_info_parts();
    let url = self.build_uri(parts);
    let response = client.get(url.clone())
                          .send()
                          .await?;
    let status_code = response.status().as_u16();
    match status_code {
      200..=299 => {
        let raw_json = response.text().await?;
        let info = serde_json::from_str(&raw_json)?;
        Ok(InfoResponse{status_code, info, raw_json, url})
      }
        _ => Err(Box::new(ResponseError::new(status_code)))
    }
  }

  /// A convenience function that wraps around request.
  pub async fn fetch(self) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();
    self.request(&client).await
  }

  pub async fn fetch_info(self) ->  Result<InfoResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    self.request_info(&client).await
  }
}