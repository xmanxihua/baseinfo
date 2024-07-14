use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct JsonResult<T>
{
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub code: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}
