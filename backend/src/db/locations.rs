//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "locations"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub gameid: Option<i32>,
    pub location_name: String,
    pub description: Option<String>,
    pub preview: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Gameid,
    LocationName,
    Description,
    Preview,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Games,
    Loot,
    Mobs,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Gameid => ColumnType::Integer.def().null(),
            Self::LocationName => ColumnType::String(None).def(),
            Self::Description => ColumnType::String(None).def().null(),
            Self::Preview => ColumnType::Binary.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Games => Entity::belongs_to(super::games::Entity)
                .from(Column::Gameid)
                .to(super::games::Column::Id)
                .into(),
            Self::Loot => Entity::has_many(super::loot::Entity).into(),
            Self::Mobs => Entity::has_many(super::mobs::Entity).into(),
        }
    }
}

impl Related<super::games::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Games.def()
    }
}

impl Related<super::loot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Loot.def()
    }
}

impl Related<super::mobs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mobs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
