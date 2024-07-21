use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_vo::SupplierVo;
use crate::repository::supplier_repo::SupplierRepo;
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};

#[derive(Clone)]
pub struct SupplierService<'a> {
    pub supplier_repo: &'a SupplierRepo<'a>,
    pub db: &'a DatabaseConnection,
}

impl <'a> SupplierService<'a> {
    pub async fn submit(&self, supplier_vo: SupplierVo) -> Result<i32, DbErr> {
        let tr = self.db.begin().await?;
        let supplier_entity: SupplierEntity = supplier_vo.into();
        let r = match supplier_entity.id {
            None => self.supplier_repo.insert(supplier_entity).await,
            Some(id) => self.supplier_repo.update(supplier_entity).await,
        };

        tr.rollback().await?;
        r
    }
}
