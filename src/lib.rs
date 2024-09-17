use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, Context};
use napi::Error;
use napi_derive::napi;

mod decode;

#[napi]
pub async fn decode_single_file(input_path: String, private_key: String) -> Result<String, Error> {
    let input = PathBuf::from_str(&input_path).context("parse input path")?;
    if !input.is_file() {
        return Err(napi::Error::from_reason("input is not a file"));
    }
    let mut ctx = decode::Context::new_with_empty_buf(input_path, private_key);
    if let Ok(res) = tokio::spawn(async move {
        if let Ok(_) = ctx.decode() {
            if let Some(v) = ctx.take_buf() {
                String::from_utf8(v).context("from utf8 error")
            } else {
                Ok(String::default())
            }
        } else {
            Err(anyhow!("decode error"))
        }
    })
    .await
    {
        res.map_err(|e| e.into())
    } else {
        Err(napi::Error::from_reason("join error"))
    }
}
