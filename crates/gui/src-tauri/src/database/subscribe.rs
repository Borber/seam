use sea_orm::entity::prelude::*;

/// 订阅记录
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "subscribe")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub live: String,
    #[sea_orm(primary_key)]
    pub rid: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
