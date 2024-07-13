use axum::extract::Json;
use serde_json::{json, Value};
use crate::bean::supplier_vo;
use crate::bean::supplier_vo::SupplierVo;
pub struct SupplierController;

impl SupplierController {
    pub async fn submit(Json(supplierVo): Json<SupplierVo>)->Json<Value> {
        println!("{:?}",supplierVo);
        Json(json!({
            "code":0,
            "data":true
        }))
    }
}