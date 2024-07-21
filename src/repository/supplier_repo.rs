use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_vo::SupplierVo;
use crate::dao::prelude::Supplier as SupplierDao;
use crate::dao::supplier;
use crate::dao::supplier::ActiveModel;

#[derive(Clone)]
pub struct SupplierRepo<'a> {
    pub db: &'a DatabaseConnection,
}

impl <'a> SupplierRepo<'a> {
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
                    Some(r) => r.try_into().map(Some).map_err(|e|DbErr::Custom(e))
                }
            });

        result
    }
}