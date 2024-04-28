use std::io::Read;

use crate::{get_reader, Base64Format};

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    // input is a file
    // if 会有一个返回类型，必须是同一个类型
    // 找共同点 使用 Box 来包装提升类型
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    // TODO: decoded data might not be string(but for this example, we assume it is)
    // let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

#[cfg(test)]
use anyhow::Result;
#[test]
fn test_encode() -> Result<()> {
    let input = "Cargo.toml";
    let format = Base64Format::UrlSafe;
    process_encode(input, format)?;

    Ok(())
}

#[test]
fn test_decode() -> Result<()> {
    let input = "fixtures/b64.txt";
    let format = Base64Format::UrlSafe;
    process_decode(input, format).unwrap();

    Ok(())
}
