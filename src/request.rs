use super::*;
use response::*;

impl Image {
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