use crate::error::Error;
use anyhow::Result;
use log::debug;
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use serde::{Deserialize, Serialize};

use super::RB;

#[crud_table(table_name:book)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub isbn: String,
    pub press: String,
    pub stock: u32,
    pub remain: u32,
}

pub async fn fuzzy_query(
    book_name: impl AsRef<str>,
    isbn: impl AsRef<str>,
    author: impl AsRef<str>,
) -> Result<Vec<Book>, Error> {
    let w = RB
        .new_wrapper()
        .like("name", book_name.as_ref())
        .like("isbn", isbn.as_ref())
        .like("author", author.as_ref());
    RB.fetch_list_by_wrapper::<Book>(w).await.map_err(|e| {
        debug!("fuzzy_query error: {e}");
        Error::BookNotExist
    })
}

pub async fn query_by_isbn(isbn: impl AsRef<str>) -> Option<Book> {
    RB.fetch_by_column::<Option<Book>, _>("isbn", isbn.as_ref())
        .await
        .ok()?
}

pub async fn add(metadata: Book) -> Result<(), Error> {
    if query_by_isbn(&metadata.isbn).await.is_some() {
        Err(Error::BookAlreadyExist)
    } else {
        RB.save(&metadata, &[])
            .await
            .map(|_r| ())
            .map_err(|_e| Error::FailedToAddBook)
    }
}

pub async fn delete(isbn: impl AsRef<str>) -> Result<(), Error> {
    if query_by_isbn(isbn.as_ref()).await.is_none() {
        Err(Error::BookNotExist)
    } else {
        RB.remove_by_column::<Book, _>("isbn", isbn.as_ref())
            .await
            .map(|_r| ())
            .map_err(|_e| Error::FailedToDeleteBook)
    }
}

pub async fn list() -> Result<Vec<Book>, Error> {
    RB.fetch_list::<Book>().await.map_err(|e| {
        debug!("{e:?}");
        Error::BookListWasEmpty
    })
}

#[crud_table(table_name:book)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BorrowBook {
    pub isbn: String,
    pub remain: u32,
}

//检查图书剩余量是否大于零，借阅的书籍是否存在
pub async fn verify_borrow(isbns: &[String]) -> Result<(), Error> {
    let w = RB
        .new_wrapper_table::<BorrowBook>()
        .in_array("isbn", isbns)
        .gt("remain", 0);
    RB.fetch_count_by_wrapper::<BorrowBook>(w)
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })?
        .eq(&(isbns.len() as u64))
        .then(|| ())
        .ok_or(Error::NoRemainBook)
}

#[crud_table(table_name:book)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBook {
    pub name: Option<String>,
    pub author: Option<String>,
    pub press: Option<String>,
    pub stock: Option<u32>,
    pub remain: Option<u32>,
}

pub async fn update(isbn: impl AsRef<str>, mut book: UpdateBook) -> Result<(), Error> {
    if let Some(stock) = book.stock {
        let rawbook = query_by_isbn(&isbn).await.ok_or(Error::DbError)?;
        let borrowed = rawbook.stock - rawbook.remain;
        if borrowed > stock {
            return Err(Error::StockIsntEnough);
        }
        book.remain = Some(stock - borrowed);
    }

    let w = RB.new_wrapper().eq("isbn", isbn.as_ref());
    RB.update_by_wrapper(&book, w, &[Skip::Value(rbson::Bson::Null)])
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })
        .map(|_| ())
}
