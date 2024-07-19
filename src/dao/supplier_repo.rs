use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use crate::entity::prelude::Supplier as SupplierDao;
use crate::entity::supplier;
use crate::entity::supplier::Model;

pub struct SupplierRepo<'a> {
    pub db: &'a DatabaseConnection,
}

impl <'a> SupplierRepo<'a> {
    pub async fn query_by_code(&self, supplier_code: String) -> Result<Option<Model>, DbErr> {
        SupplierDao::find().filter(supplier::Column::SupplierCode.eq(supplier_code)).one(self.db).await
    }
}