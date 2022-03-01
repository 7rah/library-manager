use crate::api::{new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::record::return_book as db_return_book;
use crate::types::Isbn;
use poem::web::{Data as PoemData, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ReturnReq {
    #[validate]
    isbns: Vec<Isbn>,
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
