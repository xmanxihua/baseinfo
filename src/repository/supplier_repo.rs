use std::collections::{HashMap, HashSet};

use crate::bean::page_request::PageRequest;
use crate::bean::supplier_account_entity::SupplierAccountEntity;
use crate::bean::supplier_account_param::SupplierAccountParam;
use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_param::SupplierParam;
use crate::bean::supplier_vo::SupplierVo;
use crate::constants::supplier_constants;
use crate::dao::prelude::{Supplier as SupplierDao, SupplierAccount, SupplierProduct};
use crate::dao::supplier::{ActiveModel, Entity, Model};
use crate::dao::supplier_account;
use crate::dao::{supplier, supplier_product};
use crate::repository::supplier_account_repo::SupplierAccountRepo;
use crate::repository::supplier_repo;
use sea_orm::sea_query::SelectStatement;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityName, EntityTrait, NotSet, QueryFilter,
    QuerySelect, QueryTrait, Select,
};

#[derive(Clone)]
pub struct SupplierRepo<'a> {
    pub db: &'a DatabaseConnection,
    pub supplier_account_repo: &'a SupplierAccountRepo<'a>,
}

impl<'a> SupplierRepo<'a> {
    pub async fn update(&self, supplier: SupplierEntity) -> Result<i32, DbErr> {
        if let Some(id) = supplier.id {
            SupplierDao::update(Self::adapt(supplier))
                .filter(supplier::Column::Id.eq(id))
                .exec(self.db)
                .await
                .map(|r| 1)
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
            supplier_certification: supplier
                .supplier_certification
                .map_or(NotSet, |v| Set(v.try_into().unwrap())),
        }
    }

    pub async fn insert(&self, supplier: SupplierEntity) -> Result<i32, DbErr> {
        SupplierDao::insert(Self::adapt(supplier))
            .exec(self.db)
            .await
            .map(|r| r.last_insert_id)
    }
    pub async fn query_by_code(&self, supplier_code: String) -> Result<Option<SupplierVo>, DbErr> {
        let result = SupplierDao::find()
            .filter(supplier::Column::SupplierCode.eq(supplier_code))
            .one(self.db)
            .await
            .and_then(|m| match m {
                None => Ok(None),
                Some(r) => r.try_into().map(Some).map_err(|e| DbErr::Custom(e)),
            });

        result
    }

    pub async fn query_list(
        &self,
        mut supplier_param: SupplierParam,
    ) -> Result<Vec<SupplierEntity>, DbErr> {
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
        //                                                                                         .supplier_codes(list.stream().map(Supplier::getSupplierCode).distinct().collect(Collectors.toList()))
        //                                                                                         .type(SupplierConstants.account_type.CONTACT_PERSON.getType())
        //     .build())
        // .stream().collect(Collectors.toMap(SupplierAccountEntity::getSupplierCode, Function.identity()));
        // return Util.adaptorToEntity(list, contactMap);
        if supplier_param
            .supplier_codes
            .as_ref()
            .is_some_and(|x| x.is_empty())
        {
            return Ok(vec![]);
        }

        if supplier_param
            .account_user_code
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            let supplier_accounts = SupplierAccount::find()
                .filter(
                    supplier_account::Column::UserCode
                        .eq(supplier_param.account_user_code.as_ref().unwrap()),
                )
                .all(self.db)
                .await
                .and_then(|v| {
                    let a: Result<Vec<SupplierAccountEntity>, DbErr> = v
                        .into_iter()
                        .map(|i| i.try_into().map_err(|e| DbErr::Custom(e)))
                        .collect();
                    return a;
                })?;
            let supplier_codes: HashSet<String> = supplier_accounts
                .into_iter()
                .filter(|x| x.supplier_code.as_ref().is_some_and(|y| !y.is_empty()))
                .map(|x| x.supplier_code.unwrap())
                .collect();
            if !supplier_codes.is_empty() {
                supplier_param.supplier_codes = Some(supplier_codes);
            }
        }

        let select = Self::build_select(supplier_param);

        let list = select.all(self.db).await?;

        if list.is_empty() {
            return Ok(vec![]);
        }
        let supplier_account_param = SupplierAccountParam {
            supplier_codes: Some(list.iter().map(|x| x.supplier_code.clone()).collect()),
            r#type: Some(supplier_constants::account_type::CONTACT_PERSON),
            ..SupplierAccountParam::default()
        };
        let accounts = self
            .supplier_account_repo
            .query_list(supplier_account_param)
            .await?;
        if accounts.iter().any(|x| x.supplier_code.is_none()) {
            return Err(DbErr::Custom("supplierAccount异常数据".into()));
        }

        let account_map: HashMap<String, SupplierAccountEntity> = accounts
            .into_iter()
            .map(|x| (x.supplier_code.clone().unwrap(), x))
            .collect();

        SupplierRepo::adapt_to_entity(list, account_map)
            .map_err(|e| DbErr::Custom(e.to_string()))
    }

    fn build_select(mut supplier_param: SupplierParam) -> Select<Entity> {
        let mut select = SupplierDao::find();

        select
            .apply_if(supplier_param.product_code.as_ref(), |mut select, v| {
                // 使用 sea_query 构建子查询
                let subquery = SelectStatement::new()
                    .column(supplier_product::Column::SupplierCode)
                    .from(SupplierProduct::table_ref(&supplier_product::Entity))
                    .and_where(supplier_product::Column::ProductCode.eq(v))
                    .to_owned();
                select.filter(supplier::Column::SupplierCode.in_subquery(subquery))
            })
            .apply_if(
                supplier_param.account_user_code.as_ref(),
                |mut select, v| {
                    let subquery = SelectStatement::new()
                        .column(supplier_account::Column::SupplierCode)
                        .from(SupplierAccount::table_ref(&supplier_account::Entity))
                        .and_where(supplier_account::Column::UserCode.eq(v))
                        .to_owned();
                    select.filter(supplier::Column::SupplierCode.in_subquery(subquery))
                },
            )
            .apply_if(supplier_param.one_time, |mut select, v| {
                select.filter(supplier::Column::OneTime.eq(v))
            })
            .apply_if(supplier_param.supplier_codes.as_ref(), |mut select, v| {
                select.filter(supplier::Column::SupplierCode.is_in(v))
            })
            .apply_if(supplier_param.sign_contract, |mut select, v| {
                select.filter(supplier::Column::SignContract.eq(v))
            })
            .apply_if(
                supplier_param.supplier_name_like.as_ref(),
                |mut select, v| select.filter(supplier::Column::SupplierName.contains(v)),
            )
            .apply_if(supplier_param.supplier_name.as_ref(), |mut select, v| {
                select.filter(supplier::Column::SupplierName.eq(v))
            })
            .apply_if(supplier_param.id_not_in.as_ref(), |mut select, v| {
                select.filter(supplier::Column::Id.is_not_in(v.clone()))
            })
            .apply_if(supplier_param.supplier_code.as_ref(), |mut select, v| {
                select.filter(supplier::Column::SupplierCode.eq(v))
            })
            .apply_if(supplier_param.supplier_type, |mut select, v| {
                select.filter(supplier::Column::SupplierType.eq(v))
            })
            .apply_if(supplier_param.source_system.as_ref(), |mut select, v| {
                select.filter(supplier::Column::SourceSystem.eq(v))
            })
            .apply_if(supplier_param.data_states.as_ref(), |mut select, v| {
                select.filter(supplier::Column::DataState.is_in(v.clone()))
            })
            .apply_if(supplier_param.data_state, |mut select, v| {
                select.filter(supplier::Column::DataState.eq(v))
            })
            .apply_if(supplier_param.offset(), |mut select, v| {
                select.offset(v as u64)
            })
            .apply_if(supplier_param.limit(), |mut select, v| {
                select.limit(v as u64)
            })
    }

    fn adapt_to_entity(
        pos: Vec<Model>,
        account_map: HashMap<String, SupplierAccountEntity>,
    ) -> Result<Vec<SupplierEntity>, String> {
        pos.into_iter().map(|x| x.try_into()).collect()
    }

    // private static List<SupplierEntity> adaptorToEntity(List<Supplier> pos, Map<String, SupplierAccountEntity> contactMap) {
    // return Safes.of(pos).stream().map((po) -> {
    // SupplierEntity entity = BaseAdaptor.adapt(po, SupplierEntity.class);
    // // 联系人
    // Optional.ofNullable(contactMap.get(po.getSupplierCode()))
    // .ifPresent(contact -> {
    // entity.setContactName(contact.getName());
    // entity.setContactPhone(contact.getPhone());
    // });
    //
    // return entity;
    // }).collect(Collectors.toList());
    // }
}
