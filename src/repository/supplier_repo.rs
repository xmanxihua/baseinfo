use std::collections::HashSet;

use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityName, EntityTrait, NotSet, QueryFilter, Select};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::SelectStatement;

use crate::bean::supplier_account_entity::SupplierAccountEntity;
use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_param::SupplierParam;
use crate::bean::supplier_vo::SupplierVo;
use crate::dao::{supplier, supplier_product};
use crate::dao::prelude::{Supplier as SupplierDao, SupplierAccount, SupplierProduct};
use crate::dao::supplier::{ActiveModel, Entity};
use crate::dao::supplier_account;

#[derive(Clone)]
pub struct SupplierRepo<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> SupplierRepo<'a> {
    pub async fn update(&self, supplier: SupplierEntity) -> Result<i32, DbErr> {
        if let Some(id) = supplier.id {
            SupplierDao::update(Self::adapt(supplier)).filter(supplier::Column::Id.eq(id))
                .exec(self.db).await.map(|r| 1)
        } else {
            Result::Err(DbErr::Custom("id is None".into()))
        }
    }

    fn adapt(supplier: SupplierEntity) -> ActiveModel {
        supplier::ActiveModel {
            id: NotSet,
            realm: supplier.realm.map_or(NotSet, |v| Set(v)),
            supplier_code: supplier.supplier_code.map_or(NotSet, |v| Set(v)),
            supplier_name: supplier.supplier_name.map_or(NotSet, |v| Set(v)),
            supplier_type: supplier.supplier_type.map_or(NotSet, |v| Set(v)),
            priority: supplier.priority.map_or(NotSet, |v| Set(v)),
            qa_deposit: supplier.qa_deposit.map_or(NotSet, |v| Set(v)),
            qa_deposit_paid: supplier.qa_deposit_paid.map_or(NotSet, |v| Set(v)),
            one_time: supplier.one_time.map_or(NotSet, |v| Set(v)),
            sign_contract: supplier.sign_contract.map_or(NotSet, |v| Set(v)),
            ext: supplier.ext.map_or(NotSet, |v| Set(v.try_into().unwrap())),
            data_state: supplier.data_state.map_or(NotSet, |v| Set(v)),
            create_by: supplier.create_by.map_or(NotSet, |v| Set(v)),
            update_by: supplier.update_by.map_or(NotSet, |v| Set(v)),
            create_time: supplier.create_time.map_or(NotSet, |v| Set(v)),
            update_time: supplier.update_time.map_or(NotSet, |v| Set(v)),
            supplier_nature: supplier.supplier_nature.map_or(NotSet, |v| Set(v)),
            supplier_id_code: supplier.supplier_id_code.map_or(NotSet, |v| Set(v)),
            source_system: supplier.source_system.map_or(NotSet, |v| Set(v)),
            third_supplier_code: supplier.third_supplier_code.map_or(NotSet, |v| Set(v)),
            address: supplier.address.map_or(NotSet, |v| Set(v)),
            supplier_certification: supplier.supplier_certification.map_or(NotSet, |v| Set(v.try_into().unwrap())),
        }
    }

    pub async fn insert(&self, supplier: SupplierEntity) -> Result<i32, DbErr> {
        SupplierDao::insert(Self::adapt(supplier)).exec(self.db).await.map(|r| {
            r.last_insert_id
        })
    }
    pub async fn query_by_code(&self, supplier_code: String) -> Result<Option<SupplierVo>, DbErr> {
        let result = SupplierDao::find()
            .filter(supplier::Column::SupplierCode.eq(supplier_code))
            .one(self.db).await
            .and_then(|m| {
                match m {
                    None => Ok(None),
                    Some(r) => r.try_into().map(Some).map_err(|e| DbErr::Custom(e))
                }
            });

        result
    }

    pub async fn query_list(&self, mut supplier_param: SupplierParam) -> Result<Vec<SupplierEntity>, DbErr> {
        // if (param.getSupplierCodes() != null && param.getSupplierCodes().isEmpty()) {
        //     return Lists.newArrayList();
        // }
        // //传了用户账号后,追加供应商code
        // if (StringUtils.isNotEmpty(param.getAccountUserCode())) {
        //     SupplierAccountExample accountExample = new SupplierAccountExample();
        //     accountExample.createCriteria().andUserCodeEqualTo(param.getAccountUserCode());
        //     List<SupplierAccount> supplierAccounts = supplierAccountMapper.selectByExample(accountExample);
        //     if (CollectionUtils.isNotEmpty(supplierAccounts)) {
        //         param.setSupplierCodes(supplierAccounts.stream().map(SupplierAccount::getSupplierCode).collect(Collectors.toSet()));
        //     }
        // }
        // List<Supplier> list = supplierDao.selectByExampleAndParam(Util.buildExample(param), param);
        // if (CollectionUtils.isEmpty(list)) {
        //     return Lists.newArrayList();
        // }
        // Map<String, SupplierAccountEntity> contactMap = supplierAccountRepository.queryList(SupplierAccountParam.builder()
        //                                                                                         .supplierCodes(list.stream().map(Supplier::getSupplierCode).distinct().collect(Collectors.toList()))
        //                                                                                         .type(SupplierConstants.AccountType.CONTACT_PERSON.getType())
        //     .build())
        // .stream().collect(Collectors.toMap(SupplierAccountEntity::getSupplierCode, Function.identity()));
        // return Util.adaptorToEntity(list, contactMap);
        if supplier_param.supplier_codes.as_ref().is_some_and(|x| x.is_empty()) {
            return Ok(vec![]);
        }

        if supplier_param.account_user_code.as_ref().is_some_and(|x| !x.is_empty()) {
            let supplier_accounts = SupplierAccount::find()
                .filter(supplier_account::Column::UserCode.eq(supplier_param.account_user_code.clone().unwrap()))
                .all(self.db)
                .await
                .and_then(|v| {
                    let a: Result<Vec<SupplierAccountEntity>, DbErr> = v.into_iter().map(|i| i.try_into().map_err(|e| DbErr::Custom(e))).collect();
                    return a;
                })?;
            let supplier_codes: HashSet<String> = supplier_accounts.into_iter()
                .filter(|x| x.supplier_code.as_ref().is_some_and(|y| !y.is_empty()))
                .map(|x| x.supplier_code.unwrap()).collect();
            if !supplier_codes.is_empty() {
                supplier_param.supplier_codes = Some(supplier_codes);
            }
        }

        let select = Self::build_select(supplier_param);

        let list = select.all(self.db).await?;

        if list.is_empty() {
            return Ok(vec![]);
        }




        Ok(vec![])
    }

    fn build_select(mut supplier_param: SupplierParam) -> Select<Entity> {
        let mut select = SupplierDao::find();

        if (supplier_param.product_code.is_some()) {
            // 使用 sea_query 构建子查询
            let subquery = SelectStatement::new()
                .column(supplier_product::Column::SupplierCode)
                .from(SupplierProduct::table_ref(&supplier_product::Entity))
                .and_where(supplier_product::Column::ProductCode.eq(supplier_param.product_code.unwrap()))
                .to_owned();
            select = select.filter(supplier::Column::SupplierCode.in_subquery(subquery));
        }

        if supplier_param.account_user_code.as_ref().is_some() {
            let subquery = SelectStatement::new()
                .column(supplier_account::Column::SupplierCode)
                .from(SupplierAccount::table_ref(&supplier_account::Entity))
                .and_where(supplier_account::Column::UserCode.eq(supplier_param.account_user_code.unwrap()))
                .to_owned();
            select = select.filter(supplier::Column::SupplierCode.in_subquery(subquery));
        }

        if let Some(one_time) = supplier_param.one_time {
            select = select.filter(supplier::Column::OneTime.eq(one_time));
        }

        if let Some(supplier_codes) = supplier_param.supplier_codes {
            select = select.filter(supplier::Column::SupplierCode.is_in(supplier_codes));
        }

        if let Some(x) = supplier_param.sign_contract {
            select = select.filter(supplier::Column::SignContract.eq(x));
        }

        if let Some(x) = supplier_param.supplier_name_like {
            select = select.filter(supplier::Column::SupplierName.like("%".into() + x + "%".into()));
        }
        if let Some(x) = supplier_param.supplier_name {
            select = select.filter(supplier::Column::SupplierName.eq(x));
        }
        if let Some(x) = supplier_param.id_not_in {
            select = select.filter(supplier::Column::Id.is_not_in(x));
        }
        if let Some(x) = supplier_param.supplier_code {
            select = select.filter(supplier::Column::SupplierCode.eq(x));
        }
        if let Some(x) = supplier_param.supplier_type {
            select = select.filter(supplier::Column::SupplierType.eq(x));
        }

        if let Some(x) = supplier_param.source_system {
            select = select.filter(supplier::Column::SourceSystem.eq(x));
        }
        if let Some(x) = supplier_param.data_states {
            select = select.filter(supplier::Column::DataState.is_in(x));
        }
        if let Some(x) = supplier_param.data_state {
            select = select.filter(supplier::Column::DataState.eq(x));
        }
        return select;
    }
}