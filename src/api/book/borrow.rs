use crate::api::{new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::record::borrow;
use crate::types::Isbn;
use poem::web::{Data as PoemData, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct BorrowReq {
    #[validate]
    isbns: Vec<Isbn>,
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
