use sea_orm::prelude::{DateTime, Decimal, Json};
use serde::{Deserialize, Serialize};
use crate::bean::attachment::Attachment;
use crate::bean::product_suggest_vo::ProductSuggestVo;
use crate::bean::supplier_account_entity::SupplierAccountEntity;
use crate::bean::supplier_finance_bank_entity::SupplierFinanceBankEntity;
use crate::dao::supplier::Model;

#[derive(Debug,Default,Clone)]
pub struct SupplierEntity {
    pub id: Option<i32>,
    pub realm: Option<String>,
    pub supplier_code: Option<String>,
    pub supplier_name: Option<String>,
    pub supplier_type: Option<i16>,
    pub priority: Option<i16>,
    pub qa_deposit: Option<Decimal>,
    pub qa_deposit_paid: Option<bool>,
    pub one_time: Option<bool>,
    pub sign_contract: Option<bool>,
    pub ext: Option<SupplierExt>,
    pub data_state: Option<i16>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub supplier_nature: Option<i16>,
    pub supplier_id_code: Option<String>,
    pub source_system: Option<String>,
    pub third_supplier_code: Option<String>,
    pub address: Option<String>,
    pub supplier_certification: Option<SupplierCertification>,
    pub supplier_finance_bank:Option<SupplierFinanceBankEntity>,
    pub supplier_accounts: Option<Vec<SupplierAccountEntity>>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct SupplierCertification {
    /**
     * 社会统一代码
     */
    #[serde(rename="usCode",default)]
    pub us_code: Option<String>,

    /**
     * 营业执照
     */
    #[serde(rename="businessLicense", default)]
    business_license: Option<Attachment>,

    /**
     * 其他资质
     */
    #[serde(rename="otherCertification", default)]
    other_certification: Option<Vec<Attachment>>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct SupplierExt {
    #[serde(default)]
    pub remark: Option<String>,
}

impl TryFrom<Json> for SupplierExt {
    type Error = String;

    fn try_from(value: Json) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e|e.to_string())
    }
}

impl TryInto<Json> for SupplierExt {
    type Error = String;

    fn try_into(self) -> Result<Json, Self::Error> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
}


impl TryInto<Json> for SupplierCertification {
    type Error = String;

    fn try_into(self) -> Result<Json, Self::Error> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
}

impl TryFrom<Json> for SupplierCertification {
    type Error = String;

    fn try_from(value: Json) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e|e.to_string())
    }
}

impl TryFrom<Model> for SupplierEntity {
    type Error = String;
    fn try_from(value: Model) -> Result<Self,Self::Error> {

        let ext = value.ext.try_into()?;
        let supplier_certification = value.supplier_certification.try_into()?;
        Ok(SupplierEntity {
            id: Some(value.id),
            realm: Some(value.realm),
            supplier_code: Some(value.supplier_code),
            supplier_name: Some(value.supplier_name),
            supplier_type: Some(value.supplier_type),
            priority: Some(value.priority),
            qa_deposit: Some(value.qa_deposit),
            qa_deposit_paid: Some(value.qa_deposit_paid),
            one_time: Some(value.one_time),
            sign_contract: Some(value.sign_contract),
            ext: Some(ext),
            data_state: Some(value.data_state),
            create_by: Some(value.create_by),
            update_by: Some(value.update_by),
            create_time: Some(value.create_time),
            update_time: Some(value.update_time),
            supplier_nature: Some(value.supplier_nature),
            supplier_id_code: Some(value.supplier_id_code),
            source_system: Some(value.source_system),
            third_supplier_code: Some(value.third_supplier_code),
            address: Some(value.address),
            supplier_certification: Some(supplier_certification),
            ..SupplierEntity::default()
        })
    }
}

