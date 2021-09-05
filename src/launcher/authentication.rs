use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authentication {
    pub username: String,
    pub access_token: String,
    pub client_token: String,
}
