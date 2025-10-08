// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm::DbErr;
use serde::{ser::Serializer, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaasError {
    #[error("Database error: {0}")]
    DbErr(#[from] DbErr),
    #[error("Tauri error: {0}")]
    TauriErr(#[from] tauri::Error),
}

// we must manually implement serde::Serialize
impl Serialize for KaasError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}