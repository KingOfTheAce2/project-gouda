// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "models")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub provider: String,
    pub name: String,
    pub config: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Providers {
    Ollama,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
}

impl Provider {
    pub fn all() -> Vec<Provider> {
        vec![
            Provider {
                id: "ollama".to_string(),
                name: "Ollama".to_string(),
            },
            Provider {
                id: "custom".to_string(),
                name: "Custom".to_string(),
            },
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericConfig {
    pub provider: String,
    pub config: String,
}

impl From<Model> for GenericConfig {
    fn from(model: Model) -> Self {
        Self {
            provider: model.provider,
            config: model.config,
        }
    }
}