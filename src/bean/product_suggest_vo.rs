use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct ProductSuggestVo {
    pub label:Option<String>,

    pub value:Option<String>,
}