pub mod admin;
pub mod book;
pub mod user;

use crate::error::{Error, SUCCESS_CODE};

use poem::web::Json;

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use std::str::FromStr;
use validator::Validate;

pub type JsonValue = Json<serde_json::Value>;

fn to_json<T: Serialize>(v: T) -> JsonValue {
    Json(json!(v))
}

#[derive(Debug, Serialize)]
struct ErrResp {
    code: u32,
    message: String,
}

#[derive(Debug, Serialize)]
struct SuccessResp {
    code: u32,
}

pub fn new_success_resp() -> JsonValue {
    to_json(SuccessResp { code: SUCCESS_CODE })
}

pub fn validate(data: &impl Validate) -> Result<(), Error> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::InvalidData),
    }
}

/*
pub fn new_err(e: Error) -> JsonValue {
    to_json(ErrResp {
        code: e as u32,
        message: e.to_string(),
    })
}

fn new_err_from_message(message: impl AsRef<str>) -> JsonValue {
    to_json(ErrResp {
        code: Error::InternalErr as u32,
        message: message.as_ref().to_string(),
    })
}
*/

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn from_str_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    D: Deserializer<'de>,
{
    let remote = Option::<&str>::deserialize(deserializer)?;
    if let Some(s) = remote {
        if let Ok(t) = T::from_str(s) {
            return Ok(Some(t));
        }
    }

    Ok(None)
}
