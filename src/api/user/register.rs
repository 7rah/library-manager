use crate::api::{new_success_resp, validate, JsonValue};
use crate::db::user::{add, exist, User};
use crate::error::{Error, SUCCESS_CODE};
use crate::types::{Age, Email, Introduction, Password, Sex, Sid, Username};
use crate::types::{Role, Status};
use poem::web::Json;
use poem::{handler, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate]
    username: Username,
    #[validate]
    password: Password,
    #[validate]
    sid: Sid,
    #[validate]
    email: Email,
    #[validate]
    introduction: Introduction,
    #[validate]
    age: Age,
    #[validate]
    sex: Sex,
}

#[derive(Debug, Serialize)]
pub struct RegisterResp {
    code: u32,
}

impl Default for RegisterResp {
    fn default() -> Self {
        Self { code: SUCCESS_CODE }
    }
}

#[handler]
pub async fn register(Json(req): Json<RegisterReq>) -> Result<JsonValue> {
    validate(&req)?;
    if exist(&req.email).await.is_some() {
        return Err(Error::UserAlreadyExist.into());
    }

    let user = User {
        username: req.username,
        password: req.password,
        sid: req.sid,
        email: req.email,
        introduction: req.introduction,
        age: req.age,
        sex: req.sex,
        role: Role::User,
        status: Status::Enabled,
    };

    if add(user, None).await.is_some() {
        Ok(new_success_resp())
    } else {
        Err(Error::FailedToRegister.into())
    }
}
