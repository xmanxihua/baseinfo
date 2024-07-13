//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "shop_attachment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub shop_code: String,
    pub attachment_type: String,
    pub attachment_name: String,
    pub attachment_url: String,
    pub period_type: i16,
    pub period_start: Option<Date>,
    pub period_end: Option<Date>,
    pub data_state: i16,
    pub create_by: String,
    pub update_by: String,
    pub create_time: DateTime,
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
