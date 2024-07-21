use crate::repository::supplier_repo::SupplierRepo;
use crate::service::supplier_service::SupplierService;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppStateDyn<'a> {
    pub db: &'a DatabaseConnection,
    pub supplier_repo: &'a SupplierRepo<'a>,
    pub supplier_service: &'a SupplierService<'a>,
}
