use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};

use crate::bean::app_state_dyn::AppStateDyn;
use crate::bean::json_result::JsonResult;
use crate::bean::supplier_vo::SupplierVo;

pub struct SupplierController;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Test {
    data: String,
}

impl SupplierController {
    pub async fn submit(State(state): State<AppStateDyn>, Json(supplier_vo): Json<SupplierVo>) -> Json<JsonResult<SupplierVo>> {
        let string = serde_json::to_string(&supplier_vo).unwrap();
        let s = string.as_str();
        println!("{:?}", string);

        let r = state.supplier_repo.query_by_code(supplier_vo.supplier_code.unwrap()).await;
        match r {
            Ok(x) => return Json(JsonResult {
                data: x,
                code: 0,
                msg: None,
            }),
            Err(e) => Json(JsonResult {
                data: None,
                code: -1,
                msg: Some(e.to_string()),
            })
        }
    }
}
