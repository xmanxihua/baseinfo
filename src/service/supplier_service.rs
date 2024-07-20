use crate::bean::supplier_entity::SupplierEntity;
use crate::bean::supplier_vo::SupplierVo;
use crate::repository::supplier_repo::SupplierRepo;

#[derive(Clone)]
pub struct SupplierService {
    supplier_repo: SupplierRepo,
}

impl SupplierService {
    pub async fn submit(&self, supplier_vo: SupplierVo)-> {
        let supplier_entity: SupplierEntity = supplier_vo.into();
        match supplier_entity.id {
            None => {
                self.supplier_repo.insert(supplier_entity).await;
            }
            Some(id) => {
                self.supplier_repo.update(supplier_entity).await;
            }
        }
    }
}