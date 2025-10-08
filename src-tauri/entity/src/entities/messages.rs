// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub conversation_id: i32,
    pub role: String,
    pub content: String,
    pub created_at: ChronoDateTime,
    pub prompt_token: Option<i32>,
    pub completion_token: Option<i32>,
    pub reasoning_token: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::conversations::Entity",
        from = "Column::ConversationId",
        to = "super::conversations::Column::Id"
    )]
    Conversation,
}

impl Related<super::conversations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub type Message = Model;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDTO {
    pub role: String,
    pub content: String,
}