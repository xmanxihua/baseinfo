//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bank_branch")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub branch_code: String,
    pub branch_name: String,
    pub bank_code: String,
    pub area_code: String,
    pub branch_address: String,
    pub remark: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub source_system: String,
    pub third_bank_code: String,
    pub create_time: DateTime,
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
