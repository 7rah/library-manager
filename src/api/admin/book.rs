use crate::api::admin::is_admin;
use crate::api::{new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::book::{add, delete as db_delete, update as db_update, Book, UpdateBook};
use crate::types::{Author, Bookname, Isbn, Press, Stock};
use poem::web::{Data, Json};
use poem::{handler, Result};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AddBookReq {
    #[validate]
    name: Bookname,
    #[validate]
    isbn: Isbn,
    #[validate]
    author: Author,
    #[validate]
    press: Press,
    #[validate]
    stock: Stock,
}

#[handler]
pub async fn add_book(Json(req): Json<AddBookReq>, Data(token): Data<&Token>) -> Result<JsonValue> {
    is_admin(token)?;
    validate(&req)?;

    let book = Book {
        name: req.name,
        author: req.author,
        isbn: req.isbn,
        press: req.press,
        stock: req.stock,
        remain: req.stock,
    };

    add(book).await?;

    Ok(Json(serde_json::json! ({
        "code": 20000,
    })))
}

#[derive(Debug, Deserialize, Validate)]
pub struct DeleteReq {
    isbns: Vec<Isbn>,
}

#[handler]
pub async fn delete(Json(req): Json<DeleteReq>, Data(token): Data<&Token>) -> Result<JsonValue> {
    validate(&req)?;
    is_admin(token)?;

    for isbn in &req.isbns {
        db_delete(isbn).await?;
    }

    Ok(new_success_resp())
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBookReq {
    #[validate]
    name: Option<Bookname>,
    #[validate]
    isbn: Isbn,
    #[validate]
    author: Option<Author>,
    #[validate]
    press: Option<Press>,
    #[validate]
    stock: Option<Stock>,
}

#[handler]
pub async fn update(
    Json(req): Json<UpdateBookReq>,
    Data(token): Data<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    is_admin(token)?;
    let book = UpdateBook {
        name: req.name,
        author: req.author,
        press: req.press,
        stock: req.stock,
        remain: None,
    };

    db_update(&req.isbn, book).await?;

    Ok(new_success_resp())
}
