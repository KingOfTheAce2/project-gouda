// MIT License Copyright (c) 2024-present Frank Zhang
use sea_orm_migration::cli::run_cli;
use migration::Migrator;

fn main() {
    run_cli(Migrator).await;
}