// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm_migration::prelude::*;

use entity::entities::messages;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(messages::Entity)
                    .add_column(ColumnDef::new(Alias::new("prompt_token")).integer())
                    .add_column(ColumnDef::new(Alias::new("completion_token")).integer())
                    .add_column(ColumnDef::new(Alias::new("reasoning_token")).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(messages::Entity)
                    .drop_column(Alias::new("prompt_token"))
                    .drop_column(Alias::new("completion_token"))
                    .drop_column(Alias::new("reasoning_token"))
                    .to_owned(),
            )
            .await
    }
}
