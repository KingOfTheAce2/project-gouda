// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Fixed MigratorTrait import and added ActiveModelTrait, QueryFilter, ColumnTrait
// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::{Database, DatabaseConnection, EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
use sea_orm_migration::MigratorTrait;
use std::path::Path;

use crate::errors::BearLlmAiError;
use entity::entities::{
    conversations,
    messages,
    models,
    prompts,
    settings::{self, Setting, SettingKey},
};
use migration::Migrator;

const DB_NAME: &str = "bear-llm-ai.db";

#[derive(Debug, Clone)]
pub struct Db(pub DatabaseConnection);

impl Db {
    pub async fn new(app_data_dir: &Path) -> Self {
        let db_path = app_data_dir.join(DB_NAME);

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .expect("Failed to create database directory");
            }
        }

        let db_url = format!("sqlite:{}?mode=rwc", db_path.to_str().unwrap());
        log::info!("Connecting to database at: {}", db_url);

        let conn = Database::connect(&db_url)
            .await
            .expect("failed to connect to database");

        log::info!("Running database migrations...");
        Migrator::up(&conn, None)
            .await
            .expect("failed to run migrations");

        log::info!("Database initialized successfully");
        Self(conn)
    }

    // --- Settings
    pub async fn get_settings(db: &DatabaseConnection) -> Result<serde_json::Value, BearLlmAiError> {
        let settings = settings::Entity::find().all(db).await?;
        let mut res = serde_json::Map::new();
        for setting in settings {
            res.insert(
                setting.key.to_string(),
                serde_json::from_str(&setting.value).unwrap_or(serde_json::Value::Null),
            );
        }
        Ok(serde_json::Value::Object(res))
    }

    pub async fn update_settings(
        db: &DatabaseConnection,
        payload: Vec<Setting>,
    ) -> Result<(), BearLlmAiError> {
        for setting in payload {
            let key_str = setting.key.as_str().to_string();
            let s = settings::Entity::find_by_id(key_str.clone())
                .one(db)
                .await?;
            if let Some(s) = s {
                let mut active_model: settings::ActiveModel = s.into();
                active_model.value = Set(setting.value);
                active_model.update(db).await?;
            } else {
                let new_setting = settings::ActiveModel {
                    key: Set(key_str),
                    value: Set(setting.value),
                };
                new_setting.insert(db).await?;
            }
        }
        Ok(())
    }

    pub async fn get_setting(
        db: &DatabaseConnection,
        key: SettingKey,
    ) -> Result<Setting, BearLlmAiError> {
        let key_str = key.as_str().to_string();
        let setting = settings::Entity::find_by_id(key_str).one(db).await?;
        match setting {
            Some(s) => Ok(s.into()),
            None => Err(BearLlmAiError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Setting not found".to_string(),
            ))),
        }
    }

    pub async fn get_proxy_setting(
        db: &DatabaseConnection,
    ) -> Result<Option<settings::ProxySetting>, BearLlmAiError> {
        let setting = settings::Entity::find_by_id(SettingKey::Proxy.as_str().to_string()).one(db).await?;
        match setting {
            Some(s) => {
                let proxy_setting: settings::ProxySetting = serde_json::from_str(&s.value)
                    .map_err(|_| BearLlmAiError::DbErr(sea_orm::DbErr::Json("Failed to parse proxy setting".to_string())))?;
                Ok(Some(proxy_setting))
            }
            None => Ok(None),
        }
    }

    // --- Models
    pub async fn get_models(db: &DatabaseConnection) -> Result<Vec<models::Model>, BearLlmAiError> {
        let res = models::Entity::find().all(db).await?;
        Ok(res)
    }

    pub async fn get_model(db: &DatabaseConnection, id: i32) -> Result<models::Model, BearLlmAiError> {
        let res = models::Entity::find_by_id(id).one(db).await?;
        match res {
            Some(m) => Ok(m),
            None => Err(BearLlmAiError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Model not found".to_string(),
            ))),
        }
    }

    pub async fn create_model(
        db: &DatabaseConnection,
        payload: models::Model,
    ) -> Result<models::Model, BearLlmAiError> {
        let new_model = models::ActiveModel {
            provider: Set(payload.provider.to_owned()),
            name: Set(payload.name.to_owned()),
            config: Set(payload.config.to_owned()),
            ..Default::default()
        };
        let res = new_model.insert(db).await?;
        Ok(res)
    }

    pub async fn update_model(
        db: &DatabaseConnection,
        id: i32,
        payload: models::Model,
    ) -> Result<models::Model, BearLlmAiError> {
        let model = models::Entity::find_by_id(id).one(db).await?;
        if let Some(m) = model {
            let mut active_model: models::ActiveModel = m.into();
            active_model.provider = Set(payload.provider.to_owned());
            active_model.name = Set(payload.name.to_owned());
            active_model.config = Set(payload.config.to_owned());
            let res = active_model.update(db).await?;
            Ok(res)
        } else {
            Err(BearLlmAiError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Model not found".to_string(),
            )))
        }
    }

    pub async fn delete_model(db: &DatabaseConnection, id: i32) -> Result<(), BearLlmAiError> {
        models::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn get_providers(
        _db: &DatabaseConnection,
    ) -> Result<Vec<models::Provider>, BearLlmAiError> {
        let res = models::Provider::all();
        Ok(res)
    }

    // --- Conversations
    pub async fn get_conversations(
        db: &DatabaseConnection,
    ) -> Result<Vec<conversations::Model>, BearLlmAiError> {
        let res = conversations::Entity::find().all(db).await?;
        Ok(res)
    }

    pub async fn create_conversation(
        db: &DatabaseConnection,
        payload: conversations::Model,
    ) -> Result<conversations::Model, BearLlmAiError> {
        let new_conversation = conversations::ActiveModel {
            name: Set(payload.name.to_owned()),
            model_id: Set(payload.model_id.to_owned()),
            system_message: Set(payload.system_message.to_owned()),
            options: Set(payload.options.to_owned()),
            ..Default::default()
        };
        let res = new_conversation.insert(db).await?;
        Ok(res)
    }

    pub async fn update_conversation(
        db: &DatabaseConnection,
        id: i32,
        payload: conversations::Model,
    ) -> Result<conversations::Model, BearLlmAiError> {
        let conversation = conversations::Entity::find_by_id(id).one(db).await?;
        if let Some(c) = conversation {
            let mut active_model: conversations::ActiveModel = c.into();
            active_model.name = Set(payload.name.to_owned());
            active_model.model_id = Set(payload.model_id.to_owned());
            active_model.system_message = Set(payload.system_message.to_owned());
            active_model.options = Set(payload.options.to_owned());
            let res = active_model.update(db).await?;
            Ok(res)
        } else {
            Err(BearLlmAiError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Conversation not found".to_string(),
            )))
        }
    }

    pub async fn delete_conversation(db: &DatabaseConnection, id: i32) -> Result<(), BearLlmAiError> {
        conversations::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    // --- Messages
    pub async fn get_conversation_messages(
        db: &DatabaseConnection,
        conversation_id: i32,
    ) -> Result<Vec<messages::Model>, BearLlmAiError> {
        let res = messages::Entity::find()
            .filter(messages::Column::ConversationId.eq(conversation_id))
            .all(db)
            .await?;
        Ok(res)
    }

    pub async fn create_messages(
        db: &DatabaseConnection,
        payload: Vec<messages::Model>,
    ) -> Result<Vec<messages::Model>, BearLlmAiError> {
        let new_messages = payload
            .into_iter()
            .map(|m| messages::ActiveModel {
                conversation_id: Set(m.conversation_id.to_owned()),
                role: Set(m.role.to_owned()),
                content: Set(m.content.to_owned()),
                ..Default::default()
            })
            .collect::<Vec<messages::ActiveModel>>();
        let res = messages::Entity::insert_many(new_messages)
            .exec(db)
            .await?;
        let last_insert_id = res.last_insert_id;
        let messages = messages::Entity::find()
            .filter(messages::Column::Id.gte(last_insert_id))
            .all(db)
            .await?;
        Ok(messages)
    }

    // --- Prompts
    pub async fn get_prompts(db: &DatabaseConnection) -> Result<Vec<prompts::Model>, BearLlmAiError> {
        let res = prompts::Entity::find().all(db).await?;
        Ok(res)
    }

    pub async fn create_prompt(
        db: &DatabaseConnection,
        payload: prompts::Model,
    ) -> Result<prompts::Model, BearLlmAiError> {
        let new_prompt = prompts::ActiveModel {
            name: Set(payload.name.to_owned()),
            content: Set(payload.content.to_owned()),
            ..Default::default()
        };
        let res = new_prompt.insert(db).await?;
        Ok(res)
    }

    pub async fn update_prompt(
        db: &DatabaseConnection,
        id: i32,
        payload: prompts::Model,
    ) -> Result<prompts::Model, BearLlmAiError> {
        let prompt = prompts::Entity::find_by_id(id).one(db).await?;
        if let Some(p) = prompt {
            let mut active_model: prompts::ActiveModel = p.into();
            active_model.name = Set(payload.name.to_owned());
            active_model.content = Set(payload.content.to_owned());
            let res = active_model.update(db).await?;
            Ok(res)
        } else {
            Err(BearLlmAiError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Prompt not found".to_string(),
            )))
        }
    }

    pub async fn delete_prompt(db: &DatabaseConnection, id: i32) -> Result<(), BearLlmAiError> {
        prompts::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
