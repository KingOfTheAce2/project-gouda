// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// MIT License Copyright (c) 2024-present Frank Zhang
use log::{Level, Log, Metadata, Record};
use tauri::Emitter;

pub struct FrontendLogger(pub tauri::AppHandle);

impl Log for FrontendLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}", record.level(), record.args());
            match record.level() {
                Level::Error => {
                    let _ = self.0.emit("rs-log-error", msg);
                }
                Level::Warn => {
                    let _ = self.0.emit("rs-log-warn", msg);
                }
                Level::Info => {
                    let _ = self.0.emit("rs-log-info", msg);
                }
                Level::Debug => {
                    let _ = self.0.emit("rs-log-debug", msg);
                }
                Level::Trace => {
                    let _ = self.0.emit("rs-log-trace", msg);
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn warn(tag: &str, msg: &str) {
    log::warn!("[{}] {}", tag, msg);
}