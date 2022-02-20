use crate::api::{new_success_resp, validate, JsonValue};

use super::RE_PASSWORD;

use crate::db::user::{add, exist, Role, Status, User};
use crate::error::{Error, SUCCESS_CODE};
use poem::web::Json;
use poem::{handler, Result};

use serde::{Deserialize, Serialize};

use crate::api::from_str;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(length(min = 1, max = 10))]
    username: String,
    #[validate(
        length(min = 8, max = 16),
        regex(path = "RE_PASSWORD", message = "invalid password")
    )]
    password: String,
    #[validate(range(min = 1000_0000_0000, max = 9999_9999_9999))]
    #[serde(deserialize_with = "from_str")]
    sid: u64,
    #[validate(email)]
    email: String,
    #[validate(length(min = 0, max = 200))]
    introduction: String,
    #[validate(range(min = 0, max = 100))]
    #[serde(deserialize_with = "from_str")]
    age: u32,
    #[validate(custom = "validate_sex")]
    sex: String,
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
        sid: req.sid.to_string(),
        email: req.email,
        introduction: req.introduction,
        age: req.age.to_string(),
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

fn validate_sex(sex: &str) -> Result<(), ValidationError> {
    match sex {
        "male" => Ok(()),
        "female" => Ok(()),
        "unknown" => Ok(()),
        _ => Err(ValidationError::new("invalid sex")),
    }
}
