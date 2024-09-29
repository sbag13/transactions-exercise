use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Config {
    pub input_file_path: PathBuf,
}
