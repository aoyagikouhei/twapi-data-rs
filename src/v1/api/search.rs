use crate::v1::object::{
    tweet::Tweet,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Search {
    pub statuses: Vec<Tweet>,
    pub search_metadata: SearchMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchMetadata {
    pub completed_in: f64,
    pub max_id: u64,
    pub max_id_str: String,
    pub next_results: String,
    pub query: String,
    pub count: u64,
    pub since_id: u64,
    pub since_id_str: String,
}