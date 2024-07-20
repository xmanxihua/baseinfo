use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Attachment {
    pub name: String,
    pub url: String,
}