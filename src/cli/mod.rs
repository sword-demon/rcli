mod base64;
mod csv_opts;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

use clap::Parser;

use self::{csv_opts::CsvOpts, genpass::GenPassOpts};

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv_opts::OutPutFormat,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};

/// Simple program to deal with csv
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// subcommand to show how to convert csv to other file
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> anyhow::Result<String, String> {
    // if input is "-" or file exist
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File doesn't exist".into())
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path doesn't exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File doesn't exist".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
