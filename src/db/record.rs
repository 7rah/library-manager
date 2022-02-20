use super::book::{verify_borrow, Book};
use super::user::exist;
use crate::error::Error;
use chrono::{Local, NaiveDateTime, Utc};
use log::debug;
use rbatis::crud::CRUD;
use rbatis::{crud_table, sql};

use super::RB;

#[crud_table(table_name:borrowed_book)]
pub struct BorrowedBook {
    pub isbn: String,
    pub email: String,
    pub book_name: String,
    pub borrowed_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
}

#[crud_table(table_name:return_book)]
pub struct ReturnBook {
    pub isbn: String,
    pub email: String,
    pub book_name: String,
    pub borrowed_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
}

pub async fn list_borrowed_book(email: impl AsRef<str>) -> Result<Vec<BorrowedBook>, Error> {
    RB.fetch_list_by_column::<BorrowedBook, _>("email", &[email.as_ref()])
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })
}

pub async fn list_return_book(email: impl AsRef<str>) -> Result<Vec<ReturnBook>, Error> {
    RB.fetch_list_by_column::<ReturnBook, _>("email", &[email.as_ref()])
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })
}

fn now_with_timezone() -> NaiveDateTime {
    Utc::now().with_timezone(&Local).naive_local()
}

pub async fn borrow(email: impl AsRef<str>, isbns: &[String]) -> Result<(), Error> {
    exist(&email).await.ok_or(Error::UserNotExist)?;
    verify_borrow(isbns).await?;
    let books = RB
        .fetch_list_by_column::<Book, _>("isbn", isbns)
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })?;

    let records: Vec<BorrowedBook> = books
        .into_iter()
        .map(|book| BorrowedBook {
            isbn: book.isbn,
            email: email.as_ref().to_string(),
            book_name: book.name,
            borrowed_date: now_with_timezone(),
            return_date: None,
        })
        .collect();

    RB.save_batch(&records, &[])
        .await
        .map(|r| {
            debug!("{r:?}");
        })
        .map_err(|e| {
            debug!("Error: {e}");
            Error::DbError
        })?;

    #[sql(RB, "UPDATE book SET remain=remain-1 WHERE isbn = ?")]
    async fn update_remain(isbn: &str) -> () {}

    for isbn in isbns {
        update_remain(isbn).await.map_err(|e| {
            debug!("{e}");
            Error::InternalErr
        })?;
    }

    Ok(())
}
/*
async fn verify_return(email: impl AsRef<str>, isbns: &[String]) -> Result<(), Error> {
    let w = RB
        .new_wrapper_table::<BorrowedBook>()
        .in_array("isbn", isbns)
        .eq("email", email.as_ref());
    RB.fetch_count_by_wrapper::<BorrowedBook>(w)
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })?
        .eq(&(isbns.len() as u64))
        .then(|| ())
        .ok_or(Error::BookIsNotBorrowed)
}
*/

pub async fn return_book(email: impl AsRef<str>, isbns: &[String]) -> Result<(), Error> {
    //verify_return(email.as_ref(), isbns).await?;
    //获取以前的还书记录
    let w = RB
        .new_wrapper_table::<BorrowedBook>()
        .in_array("isbn", isbns)
        .eq("email", email.as_ref());
    let v = RB
        .fetch_list_by_wrapper::<BorrowedBook>(w.clone())
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })?;

    //将记录移到已还表上
    RB.remove_by_wrapper::<BorrowedBook>(w).await.map_err(|e| {
        debug!("{e}");
        Error::DbError
    })?;
    let records: Vec<ReturnBook> = v
        .into_iter()
        .map(|record| ReturnBook {
            isbn: record.isbn,
            email: record.email,
            book_name: record.book_name,
            borrowed_date: record.borrowed_date,
            return_date: Some(now_with_timezone()),
        })
        .collect();
    RB.save_batch(&records, &[]).await.map_err(|e| {
        debug!("{e}");
        Error::DbError
    })?;

    #[sql(RB, "UPDATE book SET remain=remain+1 WHERE isbn = ?")]
    async fn update_remain(isbn: &str) -> () {}

    for isbn in isbns {
        update_remain(isbn).await.map_err(|e| {
            debug!("{e}");
            Error::InternalErr
        })?;
    }

    Ok(())
}
