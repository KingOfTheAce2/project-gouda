// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::{Database, DatabaseConnection, EntityTrait, Set};
use std::path::Path;

use crate::errors::KaasError;
use entity::entities::{
    conversations,
    messages,
    models,
    prompts,
    settings::{self, Setting, SettingKey},
};
use migration::{Migrator, MigratorTrait};

const DB_NAME: &str = "kaas.db";

#[derive(Debug, Clone)]
pub struct Db(DatabaseConnection);

impl Db {
    pub async fn new(app_data_dir: &Path) -> Self {
        let db_path = app_data_dir.join(DB_NAME);
        let db_url = format!("sqlite:{}", db_path.to_str().unwrap());
        let conn = Database::connect(&db_url)
            .await
            .expect("failed to connect to database");
        Migrator::up(&conn, None)
            .await
            .expect("failed to run migrations");
        Self(conn)
    }

    // --- Settings
    pub async fn get_settings(db: &DatabaseConnection) -> Result<serde_json::Value, KaasError> {
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
    ) -> Result<(), KaasError> {
        for setting in payload {
            let s = settings::Entity::find_by_id(setting.key.clone())
                .one(db)
                .await?;
            if let Some(s) = s {
                let mut active_model: settings::ActiveModel = s.into();
                active_model.value = Set(setting.value.to_string());
                active_model.update(db).await?;
            } else {
                let new_setting = settings::ActiveModel {
                    key: Set(setting.key.to_owned()),
                    value: Set(setting.value.to_owned()),
                };
                new_setting.insert(db).await?;
            }
        }
        Ok(())
    }

    pub async fn get_setting(
        db: &DatabaseConnection,
        key: SettingKey,
    ) -> Result<Setting, KaasError> {
        let setting = settings::Entity::find_by_id(key).one(db).await?;
        match setting {
            Some(s) => Ok(s.into()),
            None => Err(KaasError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Setting not found".to_string(),
            ))),
        }
    }

    pub async fn get_proxy_setting(
        db: &DatabaseConnection,
    ) -> Result<Option<settings::ProxySetting>, KaasError> {
        let setting = settings::Entity::find_by_id(SettingKey::Proxy).one(db).await?;
        match setting {
            Some(s) => {
                let proxy_setting: settings::ProxySetting = serde_json::from_str(&s.value)
                    .map_err(|_| KaasError::DbErr(sea_orm::DbErr::Json("Failed to parse proxy setting".to_string())))?;
                Ok(Some(proxy_setting))
            }
            None => Ok(None),
        }
    }

    // --- Models
    pub async fn get_models(db: &DatabaseConnection) -> Result<Vec<models::Model>, KaasError> {
        let res = models::Entity::find().all(db).await?;
        Ok(res)
    }

    pub async fn get_model(db: &DatabaseConnection, id: i32) -> Result<models::Model, KaasError> {
        let res = models::Entity::find_by_id(id).one(db).await?;
        match res {
            Some(m) => Ok(m),
            None => Err(KaasError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Model not found".to_string(),
            ))),
        }
    }

    pub async fn create_model(
        db: &DatabaseConnection,
        payload: models::Model,
    ) -> Result<models::Model, KaasError> {
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
    ) -> Result<models::Model, KaasError> {
        let model = models::Entity::find_by_id(id).one(db).await?;
        if let Some(m) = model {
            let mut active_model: models::ActiveModel = m.into();
            active_model.provider = Set(payload.provider.to_owned());
            active_model.name = Set(payload.name.to_owned());
            active_model.config = Set(payload.config.to_owned());
            let res = active_model.update(db).await?;
            Ok(res)
        } else {
            Err(KaasError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Model not found".to_string(),
            )))
        }
    }

    pub async fn delete_model(db: &DatabaseConnection, id: i32) -> Result<(), KaasError> {
        models::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn get_providers(
        db: &DatabaseConnection,
    ) -> Result<Vec<models::Provider>, KaasError> {
        let res = models::Provider::all();
        Ok(res)
    }

    // --- Conversations
    pub async fn get_conversations(
        db: &DatabaseConnection,
    ) -> Result<Vec<conversations::Model>, KaasError> {
        let res = conversations::Entity::find().all(db).await?;
        Ok(res)
    }

    pub async fn create_conversation(
        db: &DatabaseConnection,
        payload: conversations::Model,
    ) -> Result<conversations::Model, KaasError> {
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
    ) -> Result<conversations::Model, KaasError> {
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
            Err(KaasError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Conversation not found".to_string(),
            )))
        }
    }

    pub async fn delete_conversation(db: &DatabaseConnection, id: i32) -> Result<(), KaasError> {
        conversations::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    // --- Messages
    pub async fn get_conversation_messages(
        db: &DatabaseConnection,
        conversation_id: i32,
    ) -> Result<Vec<messages::Model>, KaasError> {
        let res = messages::Entity::find()
            .filter(messages::Column::ConversationId.eq(conversation_id))
            .all(db)
            .await?;
        Ok(res)
    }

    pub async fn create_messages(
        db: &DatabaseConnection,
        payload: Vec<messages::Model>,
    ) -> Result<Vec<messages::Model>, KaasError> {
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
    pub async fn get_prompts(db: &DatabaseConnection) -> Result<Vec<prompts::Model>, KaasError> {
        let res = prompts::Entity::find().all(db).await?;
        Ok(res)
    }

    pub async fn create_prompt(
        db: &DatabaseConnection,
        payload: prompts::Model,
    ) -> Result<prompts::Model, KaasError> {
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
    ) -> Result<prompts::Model, KaasError> {
        let prompt = prompts::Entity::find_by_id(id).one(db).await?;
        if let Some(p) = prompt {
            let mut active_model: prompts::ActiveModel = p.into();
            active_model.name = Set(payload.name.to_owned());
            active_model.content = Set(payload.content.to_owned());
            let res = active_model.update(db).await?;
            Ok(res)
        } else {
            Err(KaasError::DbErr(sea_orm::DbErr::RecordNotFound(
                "Prompt not found".to_string(),
            )))
        }
    }

    pub async fn delete_prompt(db: &DatabaseConnection, id: i32) -> Result<(), KaasError> {
        prompts::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}