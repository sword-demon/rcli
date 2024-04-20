use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> anyhow::Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File doesn't exist".into())
    }
}
