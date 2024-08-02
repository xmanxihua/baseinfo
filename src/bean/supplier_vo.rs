use crate::bean::supplier_product_entity::SupplierProductEntity;
use crate::utils::option_date_format;
use sea_orm::prelude::{DateTime, Decimal, Json};
use serde::{Deserialize, Serialize};
use crate::bean::product_suggest_vo::ProductSuggestVo;
use crate::bean::supplier_account_entity::SupplierAccountEntity;
use crate::bean::supplier_entity::{SupplierCertification, SupplierEntity, SupplierExt};
use crate::bean::supplier_finance_bank_entity::SupplierFinanceBankEntity;
use crate::dao::supplier::Model;

#[derive(Debug, Deserialize, Serialize,Default,Clone)]
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
    pub ext: Option<SupplierExt>,
    #[serde(rename = "data_state")]
    pub data_state: Option<i16>,
    #[serde(rename = "createBy")]
    pub create_by: Option<String>,
    #[serde(rename = "updateBy")]
    pub update_by: Option<String>,
    #[serde(rename = "createTime", with = "option_date_format")]
    pub create_time: Option<DateTime>,
    #[serde(rename = "updateTime", with = "option_date_format", default)]
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
    pub supplier_certification: Option<SupplierCertification>,
    #[serde(rename = "supplierFinanceBank")]
    pub supplier_finance_bank:Option<SupplierFinanceBankEntity>,
    #[serde(rename = "supplierAccounts")]
    pub supplier_accounts: Option<Vec<SupplierAccountEntity>>,
    #[serde(default)]
    pub products: Option<Vec<ProductSuggestVo>>,
}

impl Into<SupplierEntity> for SupplierVo {
    fn into(self) -> SupplierEntity {
        SupplierEntity {
            id: self.id,
            realm: self.realm,
            supplier_code: self.supplier_code,
            supplier_name: self.supplier_name,
            supplier_type: self.supplier_type,
            priority: self.priority,
            qa_deposit: self.qa_deposit,
            qa_deposit_paid: self.qa_deposit_paid,
            one_time: self.one_time,
            sign_contract: self.sign_contract,
            ext: self.ext,
            data_state: self.data_state,
            create_by: self.create_by,
            update_by: self.update_by,
            create_time: self.create_time,
            update_time: self.update_time,
            supplier_nature: self.supplier_nature,
            supplier_id_code: self.supplier_id_code,
            source_system: self.source_system,
            third_supplier_code: self.third_supplier_code,
            address: self.address,
            supplier_certification: self.supplier_certification,
            supplier_finance_bank:self.supplier_finance_bank,
            supplier_accounts:self.supplier_accounts
        }
    }
}

impl TryFrom<Model> for SupplierVo {
    type Error = String;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        let ext = value.ext.try_into()?;
        let supplier_certifications = value.supplier_certification.try_into()?;
        Ok(SupplierVo {
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
            supplier_certification: Some(supplier_certifications),
            products: None,
            ..SupplierVo::default()
        })
    }
}

