use axum::extract::{Json, State};
use axum::Extension;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::bean::app_state_dyn::AppStateDyn;
use crate::bean::json_result::JsonResult;
use crate::bean::supplier_vo::SupplierVo;
use crate::sso::bean::UserDetail;

pub struct SupplierController;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Test {
    data: String,
}

impl SupplierController {
    pub async fn submit<'a>(
        State(state): State<AppStateDyn<'a>>,
        Extension(user): Extension<Arc<UserDetail>>,
        Json(supplier_vo): Json<SupplierVo>,
    ) -> Json<JsonResult<SupplierVo>> {
        let supplier_code = supplier_vo.supplier_code.as_ref().map(|x| x.clone());

        let r = state.supplier_service.submit(supplier_vo, user).await;
        match r {
            Ok(id) => {}
            Err(e) => {
                return Json(JsonResult {
                    data: None,
                    code: -1,
                    msg: Some(e.to_string()),
                })
            }
        };

        let r = state
            .supplier_repo
            .query_by_code(supplier_code.unwrap())
            .await;
        match r {
            Ok(x) => {
                return Json(JsonResult {
                    data: x,
                    code: 0,
                    msg: None,
                })
            }
            Err(e) => Json(JsonResult {
                data: None,
                code: -1,
                msg: Some(e.to_string()),
            }),
        }
    }
}
