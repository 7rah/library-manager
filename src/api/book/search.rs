use crate::api::{to_json, validate, JsonValue};
use crate::auth::Token;
use crate::db::book::{fuzzy_query, Book};
use crate::error::SUCCESS_CODE;
use poem::web::{Data as PoemData, Json};
use poem::{handler, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
struct SearchListResp {
    code: u32,
    data: Data,
}

#[derive(Debug, Serialize)]
struct Data {
    items: Vec<Book>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SearchListReq {
    #[validate(length(min = 0, max = 50))]
    name: String,
    #[validate(length(min = 0, max = 13))]
    isbn: String,
    #[validate(length(min = 0, max = 20))]
    author: String,
}

#[handler]
pub async fn search_list(
    Json(req): Json<SearchListReq>,
    PoemData(_token): PoemData<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    let v = fuzzy_query(&req.name, &req.isbn, &req.author).await?;
    Ok(to_json(SearchListResp {
        code: SUCCESS_CODE,
        data: Data { items: v },
    }))
}
