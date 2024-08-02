use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_finance_bank_entity::SupplierFinanceBankEntity;
use crate::utils;
use crate::utils::m_assert;

// public static void supplierCheck(SupplierEntity entity){
// MAssert.notNull(entity,"瞎搞");
// MAssert.notBlank(entity.getSupplierName(),"供应商名称不能为空");
// MAssert.notNull(entity.getSupplierNature(),"供应商性质不能为空");
// MAssert.notBlank(entity.getSupplierIdCode(),"供应商证件号不能为空");
// //        MAssert.notBlank(entity.getSourceSystem(),"供应商来源系统不能为空");
// }
pub fn supplier_checker(entity: Option<&SupplierEntity>) -> Result<(), String> {
    m_assert::not_none(entity, format_args!("瞎搞"))?;
    let entity = entity.unwrap();
    m_assert::not_blank(
        entity.supplier_name.as_ref(),
        format_args!("供应商名字不能为空"),
    )?;
    m_assert::not_none(entity.supplier_nature, format_args!("供应商性质不能为空"))?;
    m_assert::not_blank(
        entity.supplier_id_code.as_ref(),
        format_args!("供应商证件号不能为空"),
    )?;
    m_assert::not_blank(
        entity.source_system.as_ref(),
        format_args!("供应商来源系统不能为空"),
    )
}

// public static void financeBankCheck(SupplierFinanceBankEntity entity){
// MAssert.notNull(entity,"瞎搞");
// MAssert.notBlank(entity.getSupplierCode(),"供应商编码不能为空");
// MAssert.notBlank(entity.getBankCode(),"银行编码不能为空");
// MAssert.notBlank(entity.getBankAccount(),"银行账号不能为空");
// MAssert.notBlank(entity.getBankAccountName(),"银行账户名不能为空");
// MAssert.notBlank(entity.getBankBranchCode(),"支行编码不能为空");
// }
pub fn finance_bank_check(entity: Option<&SupplierFinanceBankEntity>) -> Result<(), String> {
    m_assert::not_none(entity, format_args!("瞎搞"))?;
    let entity = entity.unwrap();
    m_assert::not_blank(
        entity.supplier_code.as_ref(),
        format_args!("供应商编码不能为空"),
    )?;
    m_assert::not_blank(entity.bank_code.as_ref(), format_args!("银行编码不能为空"))?;
    m_assert::not_blank(
        entity.bank_account.as_ref(),
        format_args!("银行帐号不能为空"),
    )?;
    m_assert::not_blank(
        entity.bank_account_name.as_ref(),
        format_args!("银行账户名不能为空"),
    )?;
    m_assert::not_blank(
        entity.bank_branch_code.as_ref(),
        format_args!("支行编码不能为空"),
    )
}

// public static void checkSyncVo(SupplierSyncVo param){
// MAssert.notNull(param,"瞎搞");
// MAssert.notBlank(param.getThirdSupplierCode(),"第三方供应商编码不能为空");
// MAssert.notBlank(param.getSupplierName(),"供应商名称不能为空");
// MAssert.notNull(param.getSupplierNature(),"供应商性质不能为空");
// MAssert.notBlank(param.getSupplierIdCode(),"供应商证件号不能为空");
// MAssert.notBlank(param.getBranchCode(),"银行网点编号不能为空");
// MAssert.notBlank(param.getAccountName(),"供应商收款账户名不能为空");
// MAssert.notBlank(param.getBankAccount(),"供应商收款账号不能为空");
// }
// }
