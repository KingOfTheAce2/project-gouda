// MIT License Copyright (c) 2024-present Frank Zhang
use entity::entities::prompts;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        prompts::ActiveModel {
            name: sea_orm::ActiveValue::Set("Summarize".to_string()),
            content: sea_orm::ActiveValue::Set("Summarize the following text, keeping the summary concise and to the point. The summary should be easy to read and accurately reflect the main points of the original text. Avoid adding any personal opinions or interpretations. The summary should be written in the same language as the original text.\n\n{{text}}".to_string()),
            ..Default::default()
        }.insert(db).await?;
        prompts::ActiveModel {
            name: sea_orm::ActiveValue::Set("Translate".to_string()),
            content: sea_orm::ActiveValue::Set("Translate the following text into {{language}}. The translation should be accurate and natural-sounding. Do not use any machine translation services. If you are unsure about a particular word or phrase, please leave it in the original language.\n\n{{text}}".to_string()),
            ..Default::default()
        }.insert(db).await?;
        prompts::ActiveModel {
            name: sea_orm::ActiveValue::Set("Improve Writing".to_string()),
            content: sea_orm::ActiveValue::Set("Please review the following text for grammar, spelling, and punctuation errors. Make any necessary corrections to improve the clarity and flow of the writing. Your goal is to make the text sound as natural and professional as possible.\n\n{{text}}".to_string()),
            ..Default::default()
        }.insert(db).await?;
        prompts::ActiveModel {
            name: sea_orm::ActiveValue::Set("Ask Code".to_string()),
            content: sea_orm::ActiveValue::Set("I have a coding question. I'm working on a project and I'm stuck on a specific problem. I've tried searching for a solution online, but I haven't been able to find anything that works. I'm hoping you can help me. Here's my question: {{question}}. \n\nTo give you some context, I'm using the following technologies: {{tech_stack}}. I've already tried the following solutions: {{solutions_tried}}. Here is the code I'm working with: \n\n```\n{{code}}\n```".to_string()),
            ..Default::default()
        }.insert(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        prompts::Entity::delete_many()
            .filter(prompts::Column::Name.eq("Summarize"))
            .exec(db)
            .await?;
        prompts::Entity::delete_many()
            .filter(prompts::Column::Name.eq("Translate"))
            .exec(db)
            .await?;
        prompts::Entity::delete_many()
            .filter(prompts::Column::Name.eq("Improve Writing"))
            .exec(db)
            .await?;
        prompts::Entity::delete_many()
            .filter(prompts::Column::Name.eq("Ask Code"))
            .exec(db)
            .await?;
        Ok(())
    }
}
