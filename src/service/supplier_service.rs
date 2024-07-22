use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_vo::SupplierVo;
use crate::repository::supplier_repo::SupplierRepo;
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use crate::{constants, utils};
use crate::bean::supplier_param::SupplierParam;

#[derive(Clone)]
pub struct SupplierService<'a> {
    pub supplier_repo: &'a SupplierRepo<'a>,
    pub db: &'a DatabaseConnection,
}

impl<'a> SupplierService<'a> {
    pub async fn submit(&self, supplier_vo: SupplierVo) -> Result<i32, DbErr> {
        let mut supplier_vo = supplier_vo;
        supplier_vo.supplier_type = Some(constants::product_contants::SERVICE as i16);

        let tr = self.db.begin().await?;
        let supplier_entity: SupplierEntity = supplier_vo.into();
        let r = match supplier_entity.id {
            None => self.supplier_repo.insert(supplier_entity).await,
            Some(id) => self.supplier_repo.update(supplier_entity).await,
        };

        tr.rollback().await?;
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
    fn submit_verify(supplier_entity: SupplierEntity) -> Result<(), String> {
        utils::m_assert::not_blank(supplier_entity.supplier_name.as_ref().map(|x| x.as_str()), format_args!("供应商名称不能为空"))?;
        let supplier_param = SupplierParam {
            supplier_name: supplier_entity.supplier_name.clone(),
            ..SupplierParam::default()
        };
        Ok(())
    }
}
