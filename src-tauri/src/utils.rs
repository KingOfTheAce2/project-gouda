// MIT License Copyright (c) 2024-present Frank Zhang
use base64::{
    Engine as _,
    engine::general_purpose
};
use infer;
use std::fs;
use std::path::Path;

const MAX_IMAGE_SIZE: u64 = 1024 * 1024 * 20; // 20MB

#[tauri::command]
pub fn get_image_uri(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let kind = match infer::get_from_path(path) {
        Ok(Some(kind)) => kind,
        _ => return Err("Failed to get image kind".to_string()),
    };

    if !kind.mime_type().starts_with("image/") {
        return Err("File is not an image".to_string());
    }

    let file_size = fs::metadata(path).map_err(|e| e.to_string())?.len();
    if file_size > MAX_IMAGE_SIZE {
        return Err("Image size exceeds the limit of 20MB".to_string());
    }

    let mut img_file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut img_file, &mut buf).map_err(|e| e.to_string())?;
    let encoded = general_purpose::STANDARD.encode(&buf);
    let res = format!("data:{};base64,{}", kind.mime_type(), encoded);
    Ok(res)
}
