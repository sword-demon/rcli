mod base64;
mod csv_opts;
mod genpass;

use std::path::Path;

use clap::Parser;

use self::{csv_opts::CsvOpts, genpass::GenPassOpts};

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv_opts::OutPutFormat,
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
}

fn verify_input_file(filename: &str) -> anyhow::Result<String, String> {
    // if input is "-" or file exist
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File doesn't exist".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File doesn't exist".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
