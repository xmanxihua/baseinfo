use serde::{Deserialize, Serialize};

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct SupplierFinanceBankEntity {
    /**
     * 供应商编号
     */
    #[serde(rename="supplierCode",default)]
    supplier_code:Option<String>,
    /**
     * 银行编号
     */
    #[serde(rename="bankCode",default)]
    bank_code:Option<String>,
    /**
     * 银行名称
     */
    #[serde(rename="bankName",default)]
    bank_name:Option<String>,

    /**
     * 银行支行编号
     */
    #[serde(rename="bankBranchCode",default)]
    bank_branch_code:Option<String>,
    /**
     * 银行支行名称
     */
    #[serde(rename="bankBranchName",default)]
    bank_branch_name:Option<String>,

    /**
     * 银行账号
     */
    #[serde(rename="bankAccount",default)]
    bank_account:Option<String>,

    /**
     * 银行账号名称(收款人)
     */
    #[serde(rename="bankAccountName",default)]
    bank_account_name:Option<String>,

    /**
     * 基本存款账户编号
     */
    #[serde(rename="bankAccountNumber",default)]
    base_account_number:Option<String>,

    /**
     * 税务分类
     */
    #[serde(rename="taxType",default)]
    tax_type:Option<i32>,

    /**
     * 发票类型
     */
    #[serde(rename="invoiceType",default)]
    invoice_type:Option<i32>,

    /**
     * 默认税率
     */
    #[serde(rename="taxRate",default)]
    tax_rate:Option<i32>
}