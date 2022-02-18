use crate::api::{new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::record::return_book as db_return_book;
use poem::web::{Data as PoemData, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct ReturnReq {
    #[validate(custom = "validate_isbn")]
    isbns: Vec<String>,
}

fn validate_isbn(isbns: &[String]) -> Result<(), ValidationError> {
    for i in isbns {
        if i.len() != 13 || i.parse::<u64>().is_err() {
            return Err(ValidationError::new("invaild isbn"));
        }
    }
    Ok(())
}

#[handler]
pub async fn return_book(
    Json(req): Json<ReturnReq>,
    PoemData(token): PoemData<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    db_return_book(&token.email, &req.isbns).await?;

    Ok(new_success_resp())
}
