use crate::bean::page_request::Page;
use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_param::SupplierParam;
use crate::bean::supplier_vo::SupplierVo;
use crate::repository::supplier_repo::SupplierRepo;
use crate::sso::bean::UserDetail;
use crate::{constants, utils};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use serde::de::IntoDeserializer;
use std::string::ToString;
use std::sync::Arc;
use uuid::Uuid;
use crate::service::supplier_save_checker;

#[derive(Clone)]
pub struct SupplierService<'a> {
    pub supplier_repo: &'a SupplierRepo<'a>,
    pub db: &'a DatabaseConnection,
}

impl<'a> SupplierService<'a> {
    pub async fn submit(
        &self,
        supplier_vo: SupplierVo,
        user: Arc<UserDetail>,
    ) -> Result<i32, DbErr> {
        let mut supplier_vo = supplier_vo;
        supplier_vo.supplier_type = Some(constants::product_contants::SERVICE);

        let tr = self.db.begin().await?;
        let mut supplier_entity: SupplierEntity = supplier_vo.clone().into();
        self.submit_verify(&supplier_entity)
            .await
            .map_err(|e| DbErr::Custom(e))?;

        supplier_entity.supplier_name = supplier_entity.supplier_name.map(|x| x.trim().into());

        let mut r = Ok(1);
        if supplier_entity.id.is_some() {
            supplier_entity.update_by = user.user_code.clone();
            supplier_entity.data_state.get_or_insert(1);
            r = self.supplier_repo.update(supplier_entity.clone()).await;
        } else {
            supplier_entity.supplier_code = Some("121212121".into());
            supplier_entity.data_state = Some(1);
            supplier_entity.create_by = user.user_code.clone();
            supplier_entity.update_by = user.user_code.clone();
            r = self.supplier_repo.insert(supplier_entity.clone()).await;
        }

        // 银行账号
        // Optional.ofNullable(vo.getSupplierFinanceBank()).filter(x -> StringUtils.isNotBlank(x.getBankName())).ifPresent(financeBank -> {
        //     financeBank.setSupplierCode(entity.getSupplierCode());
        //     SupplierSaveChecker.financeBankCheck(financeBank);
        //     supplierFinanceBankService.saveOrUpdate(financeBank);
        // });

        if let Some(mut finance_bank) = supplier_vo.supplier_finance_bank {
            if finance_bank
                .bank_name
                .as_ref()
                .is_some_and(|x| !x.trim().is_empty())
            {
                finance_bank.supplier_code = supplier_entity.supplier_code;
                supplier_save_checker::finance_bank_check(Some(&finance_bank)).map_err(|e|DbErr::Custom(e.to_string()))?;
            }
        }

        tr.commit().await?;
        r
    }

    // private void submitVerify(SupplierEntity entity) {
    // MAssert.notBlank(entity.getSupplierName(), "供货商名称不能为空");
    //
    // SupplierParam supplierParam = new SupplierParam();
    // supplierParam.setSupplierName(entity.getSupplierName().trim());
    // if (entity.getId() != null) {
    // supplierParam.setIdNotIn(List.of(entity.getId()));
    // }
    // SupplierEntity supplierEntity = Safes.first(supplierRepository.queryList(supplierParam));
    // if (supplierEntity != null) {
    // throw new StatusCodeException("供货商名称已存在!");
    // }
    //
    // }
    async fn submit_verify(&self, supplier_entity: &SupplierEntity) -> Result<(), String> {
        utils::m_assert::not_blank(
            supplier_entity.supplier_name.as_ref(),
            format_args!("供应商名称不能为空"),
        )?;
        let mut supplier_param = SupplierParam {
            supplier_name: supplier_entity.supplier_name.clone(),
            page: Some(Page {
                page_size: 1,
                page_no: 1,
                sorts: vec![],
            }),
            ..SupplierParam::default()
        };

        if let Some(id) = supplier_entity.id {
            supplier_param.id_not_in = Some(vec![id]);
        }
        if !self
            .supplier_repo
            .query_list(supplier_param)
            .await
            .map_err(|e| e.to_string())?
            .is_empty()
        {
            return Err("供货商名称已经存在".into());
        }
        Ok(())
    }
}
