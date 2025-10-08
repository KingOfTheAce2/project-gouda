// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::DatabaseConnection;

pub struct BearLlmAiHandle {
    pub db: DatabaseConnection,
}