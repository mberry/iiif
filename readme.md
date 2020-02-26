# IIIF

A rust client for the International Image Interoperability Framework.

For now only contains the Image API.

### Install
```toml
[dependencies]
iiif = "0.1.0"
```

### Usage

A convenience fetch function is provided and will create a new client for each
request, it is advised to create a reusable client and
pass that to the request function for multiple requests.

##### Fetch and write to file
```rust
let api = Image::new("https://ids.lib.harvard.edu/ids/iiif");
api.identifier("25286607");
let response = api.fetch()
                  .await
                  .unwrap();

// Write to foo.jpg
response.write_to_file("foo.jpg")
        .await
        .expect("Writing file to disk");
```

##### Reusable client requests
```rust
let client = Client::new();
let base = "https://ids.lib.harvard.edu/ids/iiif";
let mut images: Vec<Bytes> = Vec::new();
// Iterate through some images
for i in 25286607u32..25286617 {
  let mut api = Image::new(base);
  api.identifier(&i.to_string());
  let response = api.request(&client)
                    .await
                    .unwrap();
  images.push(response.image)
}
```