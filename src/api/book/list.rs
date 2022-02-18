use crate::api::{to_json, JsonValue};
use crate::auth::Token;
use crate::db::book::{list, Book};
use crate::error::SUCCESS_CODE;
use poem::web::Data as PoemData;
use poem::{handler, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GetListResp {
    code: u32,
    data: Data,
}

#[derive(Debug, Serialize)]
struct Data {
    items: Vec<Book>,
}

#[handler]
pub async fn get_list(PoemData(_token): PoemData<&Token>) -> Result<JsonValue> {
    let v = list().await?;

    Ok(to_json(GetListResp {
        code: SUCCESS_CODE,
        data: Data { items: v },
    }))
}
