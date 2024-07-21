use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::bean::attachment::Attachment;
use crate::dao::supplier_account::Model;
use crate::utils::option_date_format;

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierAccountEntity {
    pub id: Option<i32>,
    #[serde(rename = "supplierCode")]
    pub supplier_code: Option<String>,
    #[serde(rename = "userCode")]
    pub user_code: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub ext: Option<SupplierAccountExt>,
    #[serde(rename = "data_state")]
    pub data_state: Option<i16>,
    #[serde(rename = "createBy")]
    pub create_by: Option<String>,
    #[serde(rename = "updateBy")]
    pub update_by: Option<String>,
    #[serde(rename = "createTime", with = "option_date_format")]
    pub create_time: Option<NaiveDateTime>,
    #[serde(rename = "updateTime", with = "option_date_format")]
    pub update_time: Option<NaiveDateTime>,
    #[serde(rename = "type")]
    pub r#type: Option<Vec<i16>>,
}

impl TryFrom<Model> for SupplierAccountEntity {
    type Error = String;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        let ext = serde_json::from_value(value.ext).map_err(|e| e.to_string())?;
        let r#type = serde_json::from_value(value.r#type).map_err(|e| e.to_string())?;
        Ok(SupplierAccountEntity {
            id: Some(value.id),
            supplier_code: Some(value.supplier_code),
            user_code: Some(value.user_code),
            phone: Some(value.phone),
            name: Some(value.name),
            comment: Some(value.comment),
            ext: Some(ext),
            data_state: Some(value.data_state),
            create_by: Some(value.create_by),
            update_by: Some(value.update_by),
            create_time: Some(value.create_time.naive_local()),
            update_time: Some(value.update_time.naive_local()),
            r#type: Some(r#type),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierAccountExt {
    #[serde(rename = "idCardFront")]
    pub id_card_front: Option<Attachment>,

    #[serde(rename = "idCardBack")]
    pub id_card_back: Option<Attachment>,

    pub photo: Option<Attachment>,

    #[serde(rename = "workCard")]
    pub work_card: Option<Attachment>,
}