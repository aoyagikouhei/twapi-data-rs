use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    r#type: String,
    coordinates: Vec<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Place {
    id: String,
    url: String,
    place_type: String,
    name: String,
    full_name: String,
    country_code: String,
    country: String,
    bounding_box: Coordinates,
    attributes: serde_json::Value,
}