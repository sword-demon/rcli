use anyhow::Result;
use core::fmt;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs};

use crate::cli::OutPutFormat;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
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

pub fn process_csv(input: &str, output: String, format: OutPutFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() 将 2 个迭代器合并为一个元组的迭代器 [(header, record), ...]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    // println!("{:?}", &HashMap::from([("data", ret)]));

    let content: String = match format {
        OutPutFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutPutFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutPutFormat::Toml => toml::to_string_pretty(&HashMap::from([("data", ret)]))?,
    };

    // 写入输出文件
    fs::write(output, content)?;

    Ok(())
}

impl fmt::Display for OutPutFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
