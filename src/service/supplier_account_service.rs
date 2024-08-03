use std::collections::HashMap;
use std::ops::Add;

use sea_orm::{DatabaseConnection, DbErr};

use crate::{is_true, not_blank, not_empty, not_none};
use crate::bean::supplier_account_entity::SupplierAccountEntity;
use crate::bean::supplier_entity::SupplierEntity;
use crate::constants::supplier_constants::account_type;
use crate::utils::phone_utils;

#[derive(Debug)]
pub struct SupplierAccountService<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> SupplierAccountService<'a> {
    pub async fn submit(&self, supplier_entity: &SupplierEntity, supplier_accounts: Option<Vec<SupplierAccountEntity>>) -> Result<(), DbErr> {

        // String supplierCode = entity.getSupplierCode();
        // // 补录联系人
        // List<SupplierAccountEntity> finalAccounts = Optional.ofNullable(supplierAccounts).filter(CollectionUtils::isNotEmpty).orElseGet(ArrayList::new);
        // //        finalAccounts.add(createContactPersonAccount(entity));
        // // 提交校验
        // submitVerify(finalAccounts);
        // // UC账号处理
        // supplierAccountHandler(supplierCode, customConfig.getSupplierConfig(), finalAccounts);
        // // clear old data
        // supplierAccountRepository.deleteBySupplierCodes(List.of(supplierCode));
        // this.batchInsert(supplierCode, finalAccounts);

        let mut final_accounts = supplier_accounts.filter(|x| !x.is_empty()).unwrap_or(vec![]);

        Self::submit_verify(&final_accounts).map_err(|e| DbErr::Custom(e.to_string()))?;
        Ok(())
    }


    fn submit_verify(supplier_accounts: &Vec<SupplierAccountEntity>) -> Result<(), String> {
        not_empty!(Some(supplier_accounts), "人员信息不能为空，至少得有一个联系人");

        let mut contact: Option<&SupplierAccountEntity> = None;
        for x in supplier_accounts {
            is_true!(phone_utils::is_valid_phone_number(x.phone.as_ref()), "手机号{}不合法", x.phone.as_ref().unwrap_or(&"".into()))?;
            not_none!(x.r#type,"人员类型不能为空")?;
            not_empty!(x.r#type.as_ref(),"人员类型不能为空")?;
            if x.r#type.as_ref().is_some_and(|e| e.contains(&account_type::CONTACT_PERSON)) {
                is_true!(contact.is_none(), "联系人只能有一个")?;
                let _ = contact.insert(x);
            }
        }

        not_none!(contact,"联系人不能为空");


        let mut phone_count_map = HashMap::<&str, i32>::new();
        for x in supplier_accounts {
            let phone_number = x.phone.as_deref().unwrap_or("");
            phone_count_map.entry(phone_number)
                .or_insert(0).add(1);
        }

        let repeat_phones: Vec<&str> = phone_count_map.iter().filter(|(k, v)| **v > 1).map(|(k, v)| *k).collect();
        let repeat_phones = repeat_phones.join(",");
        not_blank!(Some(&repeat_phones),"手机号 {} 重复请检查后再提交",repeat_phones);
        Ok(())
    }

    pub fn supplier_account_handler(supplier_code: &str, final_accounts:&Vec<SupplierAccountEntity>) {

    }

    // private void supplierAccountHandler(String supplierCode, CustomConfig.SupplierConfig supplierConfig,
    // List<SupplierAccountEntity> finalAccounts) {
    // if (CollectionUtils.isEmpty(finalAccounts)) {
    // return;
    // }
    //
    // Map<String, UserPhoneAo> userPhoneAoMap = getUserPhoneAoMap(finalAccounts.stream().map(SupplierAccountEntity::getPhone).collect(Collectors.toList()));
    //
    // List<SupplierAccountEntity> insertList = new ArrayList<>();
    // List<SupplierAccountEntity> updateList = new ArrayList<>();
    //
    // finalAccounts.forEach(account -> {
    // UserPhoneAo userPhoneAo = userPhoneAoMap.get(account.getPhone());
    //
    // if (StringUtils.isBlank(account.getUserCode())) {
    // if (userPhoneAo == null) {
    // insertList.add(account);
    // } else {
    // account.setUserCode(userPhoneAo.getUserCode());
    // updateList.add(account);
    // }
    // } else {
    // updateList.add(account);
    // }
    // });
    //
    // // 注册
    // if (CollectionUtils.isNotEmpty(insertList)) {
    // this.batchRegister(supplierConfig, insertList);
    // }
    // // 修改
    // if (CollectionUtils.isNotEmpty(updateList)) {
    // this.batchSyncUser(supplierCode, updateList);
    // }
    // }
}


// private void submitVerify(List<SupplierAccountEntity> supplierAccounts) {
// CustomConfig.SupplierConfig supplierConfig = customConfig.getSupplierConfig();
// MAssert.notEmpty(supplierAccounts, "人员信息不能为空，至少得有一个联系人");
// MAssert.notNull(supplierConfig, "supplierConfig is null");
//
// SupplierAccountEntity contact = null;
// // 校验手机号合法性
// for (SupplierAccountEntity supplierAccount: supplierAccounts) {
// MAssert.isTrue(Util.isValidPhoneNumber(supplierAccount.getPhone()), "手机号" + supplierAccount.getPhone() + "不合法");
// MAssert.notNull(supplierAccount.getType(), "人员类型不能为空");
// MAssert.notEmpty(supplierAccount.getType(), "人员角色不能为空");
// if (supplierAccount.getType().contains(SupplierConstants.AccountType.CONTACT_PERSON.getType())) {
// MAssert.isTrue(contact == null, "联系人只能有一个");
// contact = supplierAccount;
// }
// }
//
// MAssert.notNull(contact, "联系人不能为空");
//
// // 校验是否存在重复手机号
// Map< String, Long > phoneCountMap = supplierAccounts.stream().collect(Collectors.groupingBy(SupplierAccountEntity::getPhone, Collectors.counting()));
// // 提示出所欲重复的手机号，用，隔开
// String repeatPhones = phoneCountMap.entrySet().stream().filter(entry -> entry.getValue() > 1).map(Map.Entry::getKey).collect(Collectors.joining(","));
// MAssert.isTrue(StringUtils.isBlank(repeatPhones), "手机号" + repeatPhones + "重复,请检查后重新提交");
// }


