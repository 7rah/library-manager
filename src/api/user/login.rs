use super::RE_PASSWORD;
use crate::api::{to_json, JsonValue};
use crate::auth::create_token;
use crate::db::user::*;
use crate::error::{Error, SUCCESS_CODE};
use poem::web::Json;
use poem::{handler, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(email)]
    email: String,
    #[validate(regex(path = "RE_PASSWORD", message = "invalid password"))]
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResp {
    code: u32,
    data: Data,
}

#[derive(Debug, Serialize)]
struct Data {
    token: String,
}

#[handler]
pub async fn login(Json(req): Json<LoginReq>) -> Result<JsonValue> {
    let user = verify(&req.email, &req.password)
        .await
        .ok_or(Error::InvalidEmailOrPassword)?;

    user.status
        .ne(&Status::Disabled)
        .then(|| ())
        .ok_or(Error::AccountWasDisabled)?;
    let token = create_token(user.email, Some(user.role))?;

    Ok(to_json(LoginResp {
        code: SUCCESS_CODE,
        data: Data { token },
    }))
}
