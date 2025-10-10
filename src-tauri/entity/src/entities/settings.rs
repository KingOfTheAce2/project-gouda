// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::{entity::prelude::*, ActiveValue, IntoActiveModel};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Hash)]
#[strum(serialize_all = "camelCase")]
pub enum SettingKey {
    General,
    Ollama,
    Appearance,
    Proxy,
}

impl SettingKey {
    pub fn as_str(&self) -> &str {
        match self {
            SettingKey::General => "general",
            SettingKey::Ollama => "ollama",
            SettingKey::Appearance => "appearance",
            SettingKey::Proxy => "proxy",
        }
    }
}

impl From<SettingKey> for String {
    fn from(key: SettingKey) -> Self {
        key.as_str().to_string()
    }
}

impl IntoActiveModel<ActiveModel> for Setting {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            key: ActiveValue::Unchanged(self.key.as_str().to_string()),
            value: ActiveValue::Set(self.value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setting {
    pub key: SettingKey,
    pub value: String,
}

impl From<Model> for Setting {
    fn from(model: Model) -> Self {
        let key = model.key.parse::<SettingKey>().unwrap_or(SettingKey::General);
        Self {
            key,
            value: model.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OllamaSetting {
    pub enabled: bool,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxySetting {
    pub enabled: bool,
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

// get max_tokens from general setting
pub async fn get_max_tokens(db: &DatabaseConnection) -> Result<u32, DbErr> {
    let setting = Entity::find_by_id(SettingKey::General.as_str().to_string()).one(db).await?;
    if let Some(s) = setting {
        let general_setting: serde_json::Value = serde_json::from_str(&s.value).unwrap();
        let max_tokens = general_setting["maxTokens"].as_u64().unwrap_or(0) as u32;
        Ok(max_tokens)
    } else {
        Ok(0)
    }
}
