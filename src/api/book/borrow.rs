use crate::api::{new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::record::borrow;
use poem::web::{Data as PoemData, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct BorrowReq {
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
pub async fn borrow_book(
    Json(req): Json<BorrowReq>,
    PoemData(token): PoemData<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    borrow(&token.email, &req.isbns).await?;
    Ok(new_success_resp())
}
