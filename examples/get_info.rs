use iiif::*;

#[tokio::main]
async fn main() {
  let mut api = Image::new("https://ids.lib.harvard.edu/ids/iiif");
  api.identifier("25286607");
  let info = api.fetch_info()
                .await;
  dbg!(info);
}

