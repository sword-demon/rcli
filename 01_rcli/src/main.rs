use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub number: u8,
}

fn main() -> Result<()> {
    // 拿到文件的内容
    let file = File::open("assets/juventus.csv")?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    for result in reader.deserialize() {
        let record: Player = result?;
        println!("{:?}", record.to_json()?);
    }
    Ok(())
}

impl Player {
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
}
