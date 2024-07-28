use serde::{Deserialize, Serialize};

#[derive(Debug, Default,Deserialize,Serialize)]
pub struct UserDetailResult {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<UserDetail>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct UserDetail {}
