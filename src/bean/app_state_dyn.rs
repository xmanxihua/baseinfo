use sea_orm::DatabaseConnection;
use crate::repository::supplier_repo::SupplierRepo;


#[derive(Clone)]
pub struct AppStateDyn
{
    pub db: DatabaseConnection,
    pub supplier_repo: SupplierRepo,
}