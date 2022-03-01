use crate::api::{to_json, JsonValue};
use crate::auth::Token;
use crate::db::user::query;
use crate::error::{Error, SUCCESS_CODE};
use crate::types::{Age, Email, Introduction, Sex, Sid, Username};
use poem::web::Data as PoemData;
use poem::{handler, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GetInfoResp {
    code: u32,
    data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    name: Username,
    email: Email,
    sid: Sid,
    age: Age,
    sex: Sex,
    roles: String,
    introduction: Introduction,
    avatar: String,
}

#[handler]
pub async fn get_info(PoemData(token): PoemData<&Token>) -> Result<JsonValue> {
    let user = query(&token.email).await.ok_or(Error::InvalidlToken)?;

    Ok(to_json(GetInfoResp {
        code: SUCCESS_CODE,
        data: Data {
            name: user.username,
            email: user.email,
            sex: user.sex,
            age: user.age,
            sid: user.sid,
            roles: user.role.to_string(),
            introduction: user.introduction,
            avatar: "https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif"
                .to_string(),
        },
    }))
}
