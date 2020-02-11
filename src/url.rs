use super::*;

impl Image {
  pub(crate) fn build_parts(&self) -> Vec<String> {
    let mut out = self.prefixes.clone();
    let mut info = vec![
      self.identifier.clone(),
      self.region.to_string(),
      self.size.to_string(),
      self.rotation.to_string(),
      format!("{}.{}", self.quality.to_string(), self.format.to_string()),
    ];
    out.append(&mut info);
    out
  }

  pub(crate) fn build_info_parts(&self) -> Vec<String> {
    let mut out = self.prefixes.clone();
    let mut info = vec![self.identifier.to_string(), "info.json".to_string()];
    out.append(&mut info);
    out.to_vec()
  }

  pub(crate) fn build_uri(self, parts: Vec<String>) -> Url {
    let mut url = Url::parse(&self.host).expect("Parsing URL Host");

    for part in parts {
      url.path_segments_mut()
          .expect("Constructing URL Parts")
          .pop_if_empty()
          .push(&part);       
    }
    url
  }
}