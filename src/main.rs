use std::path::Path;

use anyhow::Context;
use clap::Parser;
use config::Config;
use csv::{ReaderBuilder, Writer};
use engine::TxEngine;

mod config;
mod engine;
mod errors;
#[cfg(test)]
mod tests;
mod transaction_record;

// Type aliases for easier switching between different types
type ClientId = u16;
type TransactionId = u32;
type Float = f64;

fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    process_file(&config.input_file_path, std::io::stdout())
}

fn process_file(file_name: impl AsRef<Path>, writer: impl std::io::Write) -> anyhow::Result<()> {
    let mut engine = TxEngine::default();
    let mut csv_reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(file_name)
        .context("Failed to open CSV file")?;
    let transactions_iter = csv_reader.deserialize();

    transactions_iter.for_each(|tx| match tx {
        Ok(tx) => engine
            .process_tx(tx)
            .unwrap_or_else(|e| eprintln!("Failed to process transaction: {e}")),
        Err(e) => eprintln!("Failed to parse transaction: {e}"),
    });

    let mut writer = Writer::from_writer(writer);
    engine.get_clients().for_each(|client| {
        writer.serialize(client).unwrap_or_else(|e| {
            eprintln!("Failed to serialize client: {e}");
        })
    });

    Ok(())
}
