use crate::api::admin::is_admin;
use crate::api::{from_str_option, new_success_resp, validate, JsonValue};
use crate::auth::Token;
use crate::db::book::{add, delete as db_delete, update as db_update, Book, UpdateBook};
use poem::web::{Data, Json};
use poem::{handler, Result};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct AddBookReq {
    #[validate(length(min = 1, max = 50))]
    name: String,
    #[validate(range(min = 1_0000_0000_0000, max = 9_9999_9999_9999))]
    #[serde(deserialize_with = "from_str")]
    isbn: u64,
    #[validate(length(min = 0, max = 20))]
    author: String,
    #[validate(length(min = 0, max = 20))]
    press: String,
    #[validate(range(min = 0, max = 100))]
    #[serde(deserialize_with = "from_str")]
    stock: u32,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}


#[handler]
pub async fn add_book(Json(req): Json<AddBookReq>, Data(token): Data<&Token>) -> Result<JsonValue> {
    is_admin(token)?;
    validate(&req)?;

    let book = Book {
        name: req.name,
        author: req.author,
        isbn: req.isbn.to_string(),
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
    #[validate(length(min = 1, max = 50))]
    name: Option<String>,
    #[validate(range(min = 1_0000_0000_0000, max = 9_9999_9999_9999))]
    #[serde(deserialize_with = "from_str")]
    isbn: u64,
    #[validate(length(min = 0, max = 20))]
    author: Option<String>,
    #[validate(length(min = 0, max = 20))]
    press: Option<String>,
    #[validate(range(min = 0, max = 100))]
    #[serde(deserialize_with = "from_str_option")]
    stock: Option<u32>,
}

#[handler]
pub async fn update(
    Json(req): Json<UpdateBookReq>,
    Data(token): Data<&Token>,
) -> Result<JsonValue> {
    validate(&req)?;
    is_admin(token)?;

    db_update(
        &req.isbn.to_string(),
        UpdateBook {
            name: req.name,
            author: req.author,
            press: req.press,
            stock: req.stock,
            remain: None,
        },
    )
    .await?;

    Ok(new_success_resp())
}
