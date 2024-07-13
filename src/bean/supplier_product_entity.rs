use chrono::{DateTime, NaiveDateTime};
use sea_orm::prelude::{DateTimeWithTimeZone, Decimal, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct SupplierProductEntity {
    pub id: Option<i32>,
    #[serde(rename="supplierCode")]
    pub supplier_code: Option<String>,
    #[serde(rename="productCode")]
    pub product_code: Option<String>,
    pub price: Option<Decimal>,
    pub grade: Option<i16>,
    pub description: Option<String>,
    pub ext: Option<Json>,
    #[serde(rename="dataState")]
    pub data_state: Option<i16>,
    #[serde(rename="createBy")]
    pub create_by: Option<String>,
    #[serde(rename="updateBy")]
    pub update_by: Option<String>,
    #[serde(rename="createTime")]
    pub create_time: Option<NaiveDateTime>,
    #[serde(rename="updateTime")]
    pub update_time: Option<NaiveDateTime>,
}