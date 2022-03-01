use crate::api::{new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::user::{update as db_update, verify, UpdateUser};
use crate::error::Error;
use crate::types::{Age, Introduction, Password, Sex, Sid, Username};
use poem::web::{Data, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReq {
    #[validate]
    username: Username,
    #[validate]
    sid: Sid,
    #[validate]
    introduction: Introduction,
    #[validate]
    age: Age,
    #[validate]
    sex: Sex,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordReq {
    #[validate]
    old_password: Password,
    #[validate]
    new_password: Password,
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
        sid: Some(req.sid),
        introduction: Some(req.introduction),
        age: Some(req.age),
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
    verify(&token.email, &req.old_password)
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
