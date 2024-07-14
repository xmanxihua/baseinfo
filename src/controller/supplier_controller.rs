use axum::extract::Json;
use axum::http::header::MaxSizeReached;
use serde::{Deserialize, Serialize};
use crate::bean::json_result;

use crate::bean::json_result::JsonResult;
use crate::bean::supplier_vo::SupplierVo;

pub struct SupplierController;

#[derive(Serialize,Deserialize, Default,Debug)]
pub struct Test {
    data:String,
}

impl SupplierController {
    pub async fn submit(Json(supplier_vo): Json<SupplierVo>) -> Json<JsonResult<Test>> {
        let string = serde_json::to_string(&supplier_vo).unwrap();
        let s = string.as_str();
        println!("{:?}", string);

        let json_result = JsonResult{
            data: Some(Test{data:"data".into()}),
            code: 0,
            msg: None,
        };

        let string1 = serde_json::to_string(&json_result).unwrap();
        println!("{}", string1);
        let json_result2:JsonResult<Test> = serde_json::from_str(&string1).unwrap();
        return Json(json_result2);
    }
}
