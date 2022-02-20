use super::RE_PASSWORD;
use crate::api::{from_str, new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::user::{update as db_update, verify, UpdateUser};
use crate::error::Error;
use poem::web::{Data, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReq {
    #[validate(length(min = 1, max = 10))]
    username: String,
    #[validate(range(min = 1000_0000_0000, max = 9999_9999_9999))]
    #[serde(deserialize_with = "from_str")]
    sid: u64,
    #[validate(length(min = 0, max = 200))]
    introduction: String,
    #[validate(range(min = 0, max = 100))]
    #[serde(deserialize_with = "from_str")]
    age: u32,
    #[validate(custom = "validate_sex")]
    sex: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordReq {
    #[validate(
        length(min = 8, max = 16),
        regex(path = "RE_PASSWORD", message = "invalid password")
    )]
    old_password: String,
    #[validate(
        length(min = 8, max = 16),
        regex(path = "RE_PASSWORD", message = "invalid password")
    )]
    new_password: String,
}

#[handler]
pub async fn update(
    Json(req): Json<UpdateUserReq>,
    Data(token): Data<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    let user = UpdateUser {
        username: Some(req.username),
        password: None,
        sid: Some(req.sid.to_string()),
        introduction: Some(req.introduction),
        age: Some(req.age.to_string()),
        sex: Some(req.sex),
        role: None,
        status: None,
    };

    db_update(&token.email, user).await?;

    Ok(new_success_resp())
}

#[handler]
pub async fn change_password(
    Json(req): Json<ChangePasswordReq>,
    Data(token): Data<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    verify(&token.email, req.old_password)
        .await
        .ok_or(Error::InvalidPassword)?;

    let user = UpdateUser {
        username: None,
        password: Some(req.new_password),
        sid: None,
        introduction: None,
        age: None,
        sex: None,
        role: None,
        status: None,
    };

    db_update(&token.email, user).await?;

    Ok(new_success_resp())
}

fn validate_sex(sex: &str) -> Result<(), ValidationError> {
    match sex {
        "male" => Ok(()),
        "female" => Ok(()),
        "unknown" => Ok(()),
        _ => Err(ValidationError::new("invalid sex")),
    }
}
