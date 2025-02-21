//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product_image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub product_code: String,
    pub image_type: i16,
    pub image_url: String,
    pub data_state: i16,
    pub update_time: DateTimeWithTimeZone,
    pub create_time: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
