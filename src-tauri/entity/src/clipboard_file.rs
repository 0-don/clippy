//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "clipboard_file"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub id: Uuid,
    pub clipboard_id: Uuid,
    pub name: Option<String>,
    pub extension: Option<String>,
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub created_date: Option<DateTime>,
    pub modified_date: Option<DateTime>,
    pub data: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    ClipboardId,
    Name,
    Extension,
    Size,
    MimeType,
    CreatedDate,
    ModifiedDate,
    Data,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Clipboard,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Uuid.def(),
            Self::ClipboardId => ColumnType::Uuid.def(),
            Self::Name => ColumnType::String(StringLen::None).def().null(),
            Self::Extension => ColumnType::String(StringLen::None).def().null(),
            Self::Size => ColumnType::Integer.def().null(),
            Self::MimeType => ColumnType::String(StringLen::None).def().null(),
            Self::CreatedDate => ColumnType::DateTime.def().null(),
            Self::ModifiedDate => ColumnType::DateTime.def().null(),
            Self::Data => ColumnType::Blob.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Clipboard => Entity::belongs_to(super::clipboard::Entity)
                .from(Column::ClipboardId)
                .to(super::clipboard::Column::Id)
                .into(),
        }
    }
}

impl Related<super::clipboard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clipboard.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
