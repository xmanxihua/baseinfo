use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Attachment {
    pub name: String,
    pub url: String,
}