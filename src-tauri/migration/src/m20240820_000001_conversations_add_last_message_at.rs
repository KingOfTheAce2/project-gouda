// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm_migration::prelude::*;

use entity::entities::conversations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(conversations::Entity)
                    .add_column(
                        ColumnDef::new(Alias::new("last_message_at"))
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(conversations::Entity)
                    .drop_column(Alias::new("last_message_at"))
                    .to_owned(),
            )
            .await
    }
}