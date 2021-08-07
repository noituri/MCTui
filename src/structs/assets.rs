use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub objects: HashMap<String, Object>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub hash: String,
    pub size: i32,
}
