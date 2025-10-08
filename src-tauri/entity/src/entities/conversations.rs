// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conversations")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub model_id: i32,
    pub system_message: Option<String>,
    pub options: String,
    pub last_message_at: ChronoDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::messages::Entity")]
    Message,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub type Conversation = Model;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericOptions {
    pub options: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenAIOptions { // Kept for Custom provider
    pub stream: Option<bool>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<u32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub user: Option<String>,
    pub reasoning_effort: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OllamaOptions {
    pub stream: Option<bool>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<f32>,
}
