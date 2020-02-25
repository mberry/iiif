use super::*;
// use responses::*;

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

  pub async fn request_info(self, client: &Client) -> InfoResponse {
    let parts = self.build_info_parts();
    let uri = self.build_uri(parts);
    let response = client.get(uri).send().await.unwrap();
    let status_code = response.status().as_u16();
    dbg!{&response};
    let info: Info  = match status_code {
      200 => response.json().await.expect("Info payload"),
      _ => Info::default()
    };
    InfoResponse {
      status_code,
      info
    }
  }

  pub async fn fetch(self) -> Response {
    let client = reqwest::Client::new();
    self.request(&client).await
  }

  pub async fn fetch_info(self) -> InfoResponse {
    let client = reqwest::Client::new();
    self.request_info(&client).await
  }
}