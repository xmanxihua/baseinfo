use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};

use crate::bean::app_state_dyn::AppStateDyn;
use crate::bean::json_result::JsonResult;
use crate::bean::supplier_vo::SupplierVo;
use crate::entity::supplier::Model;

pub struct SupplierController;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Test {
    data: String,
}

impl SupplierController {
    pub async fn submit<'a>(State(state): State<AppStateDyn<'a>>, Json(supplier_vo): Json<SupplierVo>) -> Json<JsonResult<Model>> {
        let string = serde_json::to_string(&supplier_vo).unwrap();
        let s = string.as_str();
        println!("{:?}", string);

        let a = state.supplier_repo.query_by_code(supplier_vo.supplier_code.unwrap()).await.unwrap();
        return Json(JsonResult {
            data: a,
            code: 0,
            msg: None,
        });
    }
}
