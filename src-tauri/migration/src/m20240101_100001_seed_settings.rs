// MIT License Copyright (c) 2024-present Frank Zhang
use entity::entities::settings::{self, SettingKey};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let general_settings = serde_json::json!({
            "maxTokens": 4096,
        });
        settings::ActiveModel {
            key: sea_orm::ActiveValue::Set(SettingKey::General),
            value: sea_orm::ActiveValue::Set(general_settings.to_string()),
        }
        .insert(db)
        .await?;

        let ollama_settings = serde_json::json!({
            "enabled": true,
            "url": "http://localhost:11434",
        });
        settings::ActiveModel {
            key: sea_orm::ActiveValue::Set(SettingKey::Ollama),
            value: sea_orm::ActiveValue::Set(ollama_settings.to_string()),
        }
        .insert(db)
        .await?;

        let appearance_settings = serde_json::json!({
            "theme": "system",
            "fontSize": 14,
            "fontFamily": "Inter",
            "language": "en",
        });
        settings::ActiveModel {
            key: sea_orm::ActiveValue::Set(SettingKey::Appearance),
            value: sea_orm::ActiveValue::Set(appearance_settings.to_string()),
        }
        .insert(db)
        .await?;

        let proxy_settings = serde_json::json!({
            "enabled": false,
            "protocol": "http",
            "host": "127.0.0.1",
            "port": 7890,
        });
        settings::ActiveModel {
            key: sea_orm::ActiveValue::Set(SettingKey::Proxy),
            value: sea_orm::ActiveValue::Set(proxy_settings.to_string()),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        settings::Entity::delete_by_id(SettingKey::General)
            .exec(db)
            .await?;
        settings::Entity::delete_by_id(SettingKey::Ollama)
            .exec(db)
            .await?;
        settings::Entity::delete_by_id(SettingKey::Appearance)
            .exec(db)
            .await?;
        settings::Entity::delete_by_id(SettingKey::Proxy)
            .exec(db)
            .await?;
        Ok(())
    }
}