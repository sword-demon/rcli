use clap::Parser;
use std::{path::Path, str::FromStr};

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

#[derive(Debug, Clone, Copy)]
pub enum OutPutFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutPutFormat,
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

fn parse_format(format: &str) -> Result<OutPutFormat, anyhow::Error> {
    format.parse::<OutPutFormat>()
}

impl From<OutPutFormat> for &'static str {
    fn from(value: OutPutFormat) -> Self {
        match value {
            OutPutFormat::Json => "json",
            OutPutFormat::Yaml => "yaml",
            OutPutFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutPutFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutPutFormat::Json),
            "yaml" => Ok(OutPutFormat::Yaml),
            "toml" => Ok(OutPutFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
