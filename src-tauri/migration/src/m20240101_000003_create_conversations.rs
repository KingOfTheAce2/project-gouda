// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm_migration::prelude::*;

use crate::sea_orm::Schema;
use entity::entities::conversations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(schema.create_table_from_entity(conversations::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(conversations::Entity).to_owned())
            .await
    }
}