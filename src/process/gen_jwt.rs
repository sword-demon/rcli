use crate::{cli, Algorithm};
use anyhow::Ok;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use cli::{JWTHeader, JWTPayload};
use ring::digest;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

///
/// # 生成jwt
/// 1. 将header转换成json后使用base64加密
/// 2. 将payload转换成json之后使用base64加密
/// 3. 使用.将header和payload和secret秘钥连接起来 使用定义的加密算法加密后再使用base64生成签名
/// 4. 最后使用3个base64的字符串使用.拼接起来

pub fn process_gen_jwt(
    header: JWTHeader,
    payload: JWTPayload,
    secret: String,
) -> anyhow::Result<String> {
    let jwt_header = json!({"alg": header.alg.to_string(), "typ": header.typ});
    let jwt_payload = json!({"sub": payload.sub, "iss": payload.iss, "exp": payload.exp, "nbf": payload.nbf, "iat": payload.iat, "jti": payload.jti});

    // 转换为base64
    let header_64 = STANDARD_NO_PAD.encode(jwt_header.to_string());
    let payload_64 = STANDARD_NO_PAD.encode(jwt_payload.to_string());

    let signature = format!("{}.{}", header_64, payload_64);
    let token = match header.alg {
        Algorithm::SHA256 => {
            let mut context = digest::Context::new(&digest::SHA256);
            context.update(signature.as_bytes());
            context.update(secret.as_bytes());
            let hash = context.finish();
            // 对哈希后的值进行base64加密
            let signature_64 = STANDARD_NO_PAD.encode(hash.as_ref());
            format!("{}.{}.{}", header_64, payload_64, signature_64)
        }
        Algorithm::SHA512 => {
            let mut context = digest::Context::new(&digest::SHA512);
            context.update(signature.as_bytes());
            context.update(secret.as_bytes());
            let hash = context.finish();
            // 对哈希后的值进行base64加密
            let signature_64 = STANDARD_NO_PAD.encode(hash.as_ref());
            format!("{}.{}.{}", header_64, payload_64, signature_64)
        }
    };
    Ok(token)
}

pub fn process_validate_jwt(token: String, secret: String) -> anyhow::Result<bool> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        Err(anyhow::anyhow!("invalid token"))?;
    }

    let header_64 = parts[0];
    let payload_64 = parts[1];
    let signature_64 = parts[2];

    // 对签名base64解密
    let hmac_signature = STANDARD_NO_PAD.decode(signature_64)?;
    // 将头信息base64解密成json对象
    let header_json = STANDARD_NO_PAD.decode(header_64)?;
    // 解析成 JWTHeader 类型
    let header: JWTHeader = serde_json::from_slice(header_json.as_slice())?;

    let payload_json = STANDARD_NO_PAD.decode(payload_64)?;
    let payload: JWTPayload = serde_json::from_slice(payload_json.as_slice())?;
    // iat 签发时间大于当前服务器时间,则验证失败
    // rust 获取当前服务器时间戳
    let current_time = SystemTime::now();
    let timestamp = current_time.duration_since(UNIX_EPOCH)?;
    if payload.iat > 0 && payload.iat > timestamp.as_secs() {
        Err(anyhow::anyhow!("invalid iat"))?;
    }

    // 如果过期时间小于当前服务器时间,则验证失败
    if payload.exp > 0 && payload.exp < timestamp.as_secs() {
        Err(anyhow::anyhow!("invalid exp"))?;
    }

    // nbf 时间之前不接受处理token,验证失败
    if payload.nbf > 0 && payload.nbf > timestamp.as_secs() {
        Err(anyhow::anyhow!("invalid nbf"))?;
    }

    let signature_concat = format!("{}.{}", header_64, payload_64);

    let result = match header.alg {
        Algorithm::SHA256 => {
            let mut context = digest::Context::new(&digest::SHA256);
            context.update(signature_concat.as_bytes());
            context.update(secret.as_bytes());
            let hash = context.finish();

            // 如何判断 hmac_256_signature 和 hash 是否 哈希相等
            // println!("{:?}", hash.as_ref());
            // println!("{:?}", hmac_signature.as_slice());

            hash.as_ref() == hmac_signature.as_slice()
        }
        Algorithm::SHA512 => {
            let mut context = digest::Context::new(&digest::SHA512);
            context.update(signature_concat.as_bytes());
            context.update(secret.as_bytes());
            let hash = context.finish();

            // 如何判断 hmac_512_signature 和 hash 是否 哈希相等
            // println!("{:?}", hash.as_ref());
            // println!("{:?}", hmac_signature.as_slice());

            hash.as_ref() == hmac_signature.as_slice()
        }
    };

    Ok(result)
}
