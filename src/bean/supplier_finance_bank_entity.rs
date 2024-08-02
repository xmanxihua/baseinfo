use crate::dao::supplier_finance_bank;
use crate::dao::supplier_finance_bank::{ActiveModel, Model};
use sea_orm::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct SupplierFinanceBankEntity {
    pub id: Option<i32>,
    /**
     * 供应商编号
     */
    #[serde(rename = "supplierCode", default)]
    pub supplier_code: Option<String>,
    /**
     * 银行编号
     */
    #[serde(rename = "bankCode", default)]
    pub bank_code: Option<String>,
    /**
     * 银行名称
     */
    #[serde(rename = "bankName", default)]
    pub bank_name: Option<String>,

    /**
     * 银行支行编号
     */
    #[serde(rename = "bankBranchCode", default)]
    pub bank_branch_code: Option<String>,
    /**
     * 银行支行名称
     */
    #[serde(rename = "bankBranchName", default)]
    pub bank_branch_name: Option<String>,

    /**
     * 银行账号
     */
    #[serde(rename = "bankAccount", default)]
    pub bank_account: Option<String>,

    /**
     * 银行账号名称(收款人)
     */
    #[serde(rename = "bankAccountName", default)]
    pub bank_account_name: Option<String>,

    /**
     * 基本存款账户编号
     */
    #[serde(rename = "bankAccountNumber", default)]
    pub base_account_number: Option<String>,

    /**
     * 税务分类
     */
    #[serde(rename = "taxType", default)]
    pub tax_type: Option<i32>,

    /**
     * 发票类型
     */
    #[serde(rename = "invoiceType", default)]
    pub invoice_type: Option<i32>,

    /**
     * 默认税率
     */
    #[serde(rename = "taxRate", default)]
    pub tax_rate: Option<i32>,
}
impl Into<supplier_finance_bank::ActiveModel> for SupplierFinanceBankEntity {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: Default::default(),
            supplier_code: self.supplier_code.map_or(NotSet, |x| Set(x)),
            bank_code: self.bank_code.map_or(NotSet, |x| Set(x)),
            bank_branch_code: self.bank_branch_code.map_or(NotSet, |x| Set(x)),
            bank_account: self.bank_account.map_or(NotSet, |x| Set(x)),
            bank_account_name: self.bank_account_name.map_or(NotSet, |x| Set(x)),
            create_time: NotSet,
            update_time: NotSet,
            base_account_number: self.base_account_number.map_or(NotSet, |x| Set(x)),
            tax_type: self.tax_type.map_or(NotSet, |x| Set(x as i16)),
            invoice_type: self.invoice_type.map_or(NotSet, |x| Set(x as i16)),
            tax_rate: self.tax_rate.map_or(NotSet, |x| Set(x as i16)),
        }
    }
}
