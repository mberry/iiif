//! Helper functions

// Limit float resolution down to 3 decimal places
pub(crate) fn format_floats<T: std::fmt::Display>(floats: Vec<T>) -> Vec<String> {
  floats.iter()
        .map(|float| format!("{:.5}", float.to_string()))
        .collect()
}

// Convert coords to iiif parameter string
pub(crate) fn join_coords<T: ToString>(x: T, y: T, w: T, h: T) -> String {
  [x, y, w, h].iter()
              .map(|s| s.to_string())
              .collect::<Vec<String>>()
              .join(",")
}