use crate::bean::supplier_product_entity::SupplierProductEntity;
use crate::utils::option_date_format;
use sea_orm::prelude::{DateTime, Decimal, Json};
use serde::{Deserialize, Serialize};
use crate::bean::product_suggest_vo::ProductSuggestVo;

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierVo {
    pub id: Option<i32>,
    pub realm: Option<String>,
    #[serde(rename = "supplierCode")]
    pub supplier_code: Option<String>,
    #[serde(rename = "supplierName")]
    pub supplier_name: Option<String>,
    #[serde(rename = "supplierType")]
    pub supplier_type: Option<i16>,
    pub priority: Option<i16>,
    #[serde(rename = "qaDeposit")]
    pub qa_deposit: Option<Decimal>,
    #[serde(rename = "qaDepositPaid")]
    pub qa_deposit_paid: Option<bool>,
    #[serde(rename = "oneTime")]
    pub one_time: Option<bool>,
    #[serde(rename = "signContract")]
    pub sign_contract: Option<bool>,
    pub ext: Option<Json>,
    #[serde(rename = "dataState")]
    pub data_state: Option<i16>,
    #[serde(rename = "createBy")]
    pub create_by: Option<String>,
    #[serde(rename = "updateBy")]
    pub update_by: Option<String>,
    #[serde(rename = "createTime", with = "option_date_format")]
    pub create_time: Option<DateTime>,
    #[serde(rename = "updateTime", with = "option_date_format",default)]
    pub update_time: Option<DateTime>,
    #[serde(rename = "supplierNature")]
    pub supplier_nature: Option<i16>,
    #[serde(rename = "supplierIdCode")]
    pub supplier_id_code: Option<String>,
    #[serde(rename = "sourceSystem")]
    pub source_system: Option<String>,
    #[serde(rename = "thirdSupplierCode")]
    pub third_supplier_code: Option<String>,
    pub address: Option<String>,
    #[serde(rename = "supplierCertification")]
    pub supplier_certification: Option<Json>,
    pub products: Option<Vec<ProductSuggestVo>>,
}
