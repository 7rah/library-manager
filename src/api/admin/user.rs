use crate::api::admin::is_admin;
use crate::api::{from_str_option, new_success_resp, to_json, validate, JsonValue};
use crate::auth::Token;
use crate::db::user::{list as db_list, update as db_update, UpdateUser};
use crate::error::SUCCESS_CODE;
use crate::types::{Email, Password, Role, Sid, Status, Username};
use poem::web::{Data as PoemData, Json};
use poem::{handler, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
struct GetUserListResp {
    code: u32,
    data: Data,
}

#[derive(Debug, Serialize)]
struct Data {
    users: Vec<User>,
}

#[derive(Debug, Serialize)]
struct User {
    name: Username,
    sid: Sid,
    email: Email,
    role: String,
    status: String,
}

#[handler]
pub async fn list(PoemData(token): PoemData<&Token>) -> Result<JsonValue> {
    is_admin(token)?;

    let v = db_list().await?;
    let users: Vec<User> = v
        .into_iter()
        .map(|user| User {
            name: user.username,
            sid: user.sid,
            email: user.email,
            role: user.role.to_string(),
            status: user.status.to_string(),
        })
        .collect();

    Ok(to_json(GetUserListResp {
        code: SUCCESS_CODE,
        data: Data { users },
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReq {
    #[validate]
    password: Option<Password>,
    #[serde(deserialize_with = "from_str_option", default)]
    status: Option<Status>,
    role: Option<Role>,
    email: Email,
}

#[handler]
pub async fn update(
    Json(req): Json<UpdateUserReq>,
    PoemData(token): PoemData<&Token>,
) -> Result<JsonValue> {
    is_admin(token)?;
    validate(&req)?;

    let user = UpdateUser {
        age: None,
        introduction: None,
        sex: None,
        status: req.status,
        role: req.role.clone(),
        password: req.password,
        sid: None,
        username: None,
    };

    db_update(&req.email, user).await?;

    Ok(new_success_resp())
}
