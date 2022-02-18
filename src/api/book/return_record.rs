use crate::api::{to_json, JsonValue};
use crate::auth::Token;
use crate::db::record::list_return_book;
use crate::error::SUCCESS_CODE;
use poem::web::Data as PoemData;
use poem::{handler, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ListReturnResp {
    code: u32,
    data: Data,
}

#[derive(Debug, Serialize)]
struct Data {
    items: Vec<Item>,
}

#[derive(Debug, Serialize)]
struct Item {
    name: String,
    isbn: String,
    borrowed_date: String,
    return_date: String,
}

#[handler]
pub async fn list_return(PoemData(token): PoemData<&Token>) -> Result<JsonValue> {
    let v = list_return_book(&token.email).await?;

    let items: Vec<Item> = v
        .into_iter()
        .map(|book| Item {
            name: book.book_name,
            isbn: book.isbn,
            borrowed_date: book.borrowed_date.to_string(),
            return_date: book.return_date.map_or("".to_string(), |t| t.to_string()),
        })
        .collect();

    Ok(to_json(ListReturnResp {
        code: SUCCESS_CODE,
        data: Data { items },
    }))
}
