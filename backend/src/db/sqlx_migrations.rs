//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "_sqlx_migrations"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub version: i64,
    pub description: String,
    pub installed_on: DateTimeWithTimeZone,
    pub success: bool,
    pub checksum: Vec<u8>,
    pub execution_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Version,
    Description,
    InstalledOn,
    Success,
    Checksum,
    ExecutionTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Version,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Version => ColumnType::BigInteger.def(),
            Self::Description => ColumnType::Text.def(),
            Self::InstalledOn => ColumnType::TimestampWithTimeZone.def(),
            Self::Success => ColumnType::Boolean.def(),
            Self::Checksum => ColumnType::Binary.def(),
            Self::ExecutionTime => ColumnType::BigInteger.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
