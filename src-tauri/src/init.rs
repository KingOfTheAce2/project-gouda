// MIT License Copyright (c) 2024-present Frank Zhang
use crate::{
    core::handle::KaasHandle,
    services::db::Db,
};
use tauri::{
    App,
    Manager,
    Wry,
};

pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let db = tauri::async_runtime::block_on(async {
        let app_data_dir = handle
            .path()
            .app_data_dir()
            .expect("failed to resolve app data dir");
        Db::new(&app_data_dir).await
    });
    handle.manage(KaasHandle { db });
    Ok(())
}