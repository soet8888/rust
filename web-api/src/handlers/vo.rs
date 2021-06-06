use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hub {
    pub status: String,
    pub version: String,
    pub total_mem: String,
    pub free_mem: String,
    pub used_mem: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientResponse {
    pub status: String,
    pub data: String,
    pub message: String,
}
