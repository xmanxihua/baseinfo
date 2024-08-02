use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct ProductSuggestVo {
    pub label:Option<String>,

    pub value:Option<String>,
}