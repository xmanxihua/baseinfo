use sea_orm::DatabaseConnection;
use crate::dao::supplier_repo::SupplierRepo;

#[derive(Clone)]
pub struct AppStateDyn<'a> {
    pub db: DatabaseConnection,
    pub supplier_repo: &'a SupplierRepo<'a>,
}