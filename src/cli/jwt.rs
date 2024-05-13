use std::fmt::{self};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{process_gen_jwt, process_validate_jwt, CmdExector};

#[derive(Debug, Parser)]
pub struct GenJwt {
    #[command(flatten)]
    pub header: JWTHeader,
    #[command(flatten)]
    pub payload: JWTPayload,
    #[arg(long)]
    pub secret: String,
}

impl CmdExector for GenJwt {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_gen_jwt(self.header, self.payload, self.secret)?;
        println!("{}", token);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct ValidateJwt {
    #[arg(long)]
    pub token: String,
    #[arg(long)]
    pub secret: String,
}

impl CmdExector for ValidateJwt {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_validate_jwt(self.token, self.secret)?;
        println!("validate result: {}", result);
        Ok(())
    }
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JWTHeader {
    #[arg(long, default_value = "SHA256", value_parser = parse_algorithm)]
    pub alg: Algorithm,
    #[arg(long, default_value_t = String::from("JWT"))]
    pub typ: String,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    SHA256,
    SHA512,
}

fn parse_algorithm(algo: &str) -> Result<Algorithm, anyhow::Error> {
    algo.parse()
}

impl std::str::FromStr for Algorithm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SHA256" => Ok(Algorithm::SHA256),
            "SHA512" => Ok(Algorithm::SHA512),
            _ => Err(anyhow::anyhow!("invalid algorithm")),
        }
    }
}

impl From<Algorithm> for &'static str {
    fn from(value: Algorithm) -> Self {
        match value {
            Algorithm::SHA256 => "SHA256",
            Algorithm::SHA512 => "SHA512",
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Clone, Parser, Deserialize, Serialize)]
pub struct JWTPayload {
    /// 主题,所面向的用户
    #[arg(long)]
    pub sub: String,
    /// 签发者
    #[arg(long)]
    pub iss: String,
    /// 过期时间,这里是时间戳,超过这个时间就会变得无效
    #[arg(long, value_parser = time_in_range)]
    pub exp: u64,
    /// 生效时间,时间戳格式,在这个时间之前的是不被接受的 即 nbf ~ exp 之间的才是生效时间
    #[arg(long, value_parser = time_in_range)]
    pub nbf: u64,
    /// 签发时间,时间戳格式, jwt 创建的时间
    #[arg(long, value_parser = time_in_range)]
    pub iat: u64,
    /// jwt 的唯一标识
    #[arg(long)]
    pub jti: Option<String>,
}

fn time_in_range(input: &str) -> Result<u64, String> {
    let time = input
        .parse::<u64>()
        .map_err(|_| "time must be a number".to_string())?;
    if time > 0 {
        Ok(time)
    } else {
        Err("time must be greater than 0".to_string())
    }
}
