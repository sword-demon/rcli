use std::str::FromStr;

use clap::Parser;

use crate::{process_csv, CmdExector};

use super::verify_file;

#[derive(Debug, Clone, Copy)]
pub enum OutPutFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
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

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, output, self.format)
    }
}
