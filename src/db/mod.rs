use log::info;
use rbatis::executor::Executor;
use rbatis::rbatis::Rbatis;

use crate::db::user::{add, exist, Role};

pub mod book;
pub mod record;
pub mod user;

lazy_static::lazy_static! {
    static ref RB:Rbatis=Rbatis::new();
}

pub fn encode_password(password: impl AsRef<str>) -> String {
    format!("{:x}", md5::compute(password.as_ref().as_bytes()))
}

const MYSQL_TABLE_USER: &str = "
CREATE TABLE IF NOT EXISTS `user`(
    `username` VARCHAR(255),
    `password` VARCHAR(255),
    `email` VARCHAR(255),
    `sid` VARCHAR(255),
    `introduction` VARCHAR(255),
    `age` VARCHAR(255),
    `sex` VARCHAR(255),
    `role` TINYINT,
    `status` TINYINT,
    PRIMARY KEY ( `email` )
)ENGINE=InnoDB DEFAULT CHARSET=utf8;";

const MYSQL_TABLE_BOOK: &str = "
CREATE TABLE IF NOT EXISTS `book`(
    `name` VARCHAR(255),
    `author` VARCHAR(255),
    `isbn` VARCHAR(13),
    `press` VARCHAR(255),
    `remain` INT,
    `stock` INT,
    PRIMARY KEY ( `isbn` )
)ENGINE=InnoDB DEFAULT CHARSET=utf8;";

const MYSQL_TABLE_BORROWED_BOOK: &str = "CREATE TABLE IF NOT EXISTS `borrowed_book`(
    `id`   BIGINT NOT NULL AUTO_INCREMENT,
    `isbn` VARCHAR(13),
    `email` VARCHAR(255),
    `book_name` VARCHAR(255),
    `borrowed_date` DATETIME,
    `return_date` DATETIME,

    PRIMARY KEY ( `id` ),

    FOREIGN KEY (`isbn`)
    REFERENCES book(`isbn`)
    ON DELETE CASCADE,

    FOREIGN KEY (`email`)
    REFERENCES user(`email`)
    ON DELETE CASCADE

)ENGINE=InnoDB DEFAULT CHARSET=utf8;";

const MYSQL_TABLE_RETURN_BOOK: &str = "CREATE TABLE IF NOT EXISTS `return_book`(
    `id`   BIGINT NOT NULL AUTO_INCREMENT,
    `isbn` VARCHAR(13),
    `email` VARCHAR(255),
    `book_name` VARCHAR(255),
    `borrowed_date` DATETIME,
    `return_date` DATETIME,

    PRIMARY KEY ( `id` ),

    FOREIGN KEY (`isbn`)
    REFERENCES book(`isbn`)
    ON DELETE CASCADE,

    FOREIGN KEY (`email`)
    REFERENCES user(`email`)
    ON DELETE CASCADE

)ENGINE=InnoDB DEFAULT CHARSET=utf8;";

const SQLITE_TABLE_USER: &str = "CREATE TABLE IF NOT EXISTS `user`(
    `username` VARCHAR(255),
    `password` VARCHAR(255),
    `email` VARCHAR(255),
    `sid` VARCHAR(255),
    `introduction` VARCHAR(255),
    `age` VARCHAR(255),
    `sex` VARCHAR(255),
    `role` TINYINT,
    `status` TINYINT,
    PRIMARY KEY ( `email` )
)";

const SQLITE_TABLE_BOOK: &str = "CREATE TABLE IF NOT EXISTS `book`(
    `name` VARCHAR(255),
    `author` VARCHAR(255),
    `isbn` VARCHAR(13),
    `press` VARCHAR(255),
    `remain` INT,
    `stock` INT,
    PRIMARY KEY ( `isbn` )
)";

const SQLITE_TABLE_BORROWED_BOOK: &str = "CREATE TABLE IF NOT EXISTS `borrowed_book`(
    `id`   INTEGER PRIMARY KEY ,
    `isbn` VARCHAR(13),
    `email` VARCHAR(255),
    `book_name` VARCHAR(255),
    `borrowed_date` DATETIME,
    `return_date` DATETIME,
    
    FOREIGN KEY (`isbn`)
    REFERENCES book(`isbn`)
    ON DELETE CASCADE,

    FOREIGN KEY (`email`)
    REFERENCES user(`email`)
    ON DELETE CASCADE
)";

const SQLITE_TABLE_RETURN_BOOK: &str = "CREATE TABLE IF NOT EXISTS `return_book`(
    `id`   INTEGER PRIMARY KEY ,
    `isbn` VARCHAR(13),
    `email` VARCHAR(255),
    `book_name` VARCHAR(255),
    `borrowed_date` DATETIME,
    `return_date` DATETIME,
    
    FOREIGN KEY (`isbn`)
    REFERENCES book(`isbn`)
    ON DELETE CASCADE,

    FOREIGN KEY (`email`)
    REFERENCES user(`email`)
    ON DELETE CASCADE
)";

pub async fn init_db(addr: &str) {
    info!("link db {addr}");
    RB.link(addr).await.unwrap();

    if &addr[0..6] == "sqlite" {
        info!("create sqlite table if not exist");
        RB.exec(SQLITE_TABLE_USER, vec![]).await.unwrap();
        RB.exec(SQLITE_TABLE_BOOK, vec![]).await.unwrap();
        RB.exec(SQLITE_TABLE_BORROWED_BOOK, vec![]).await.unwrap();
        RB.exec(SQLITE_TABLE_RETURN_BOOK, vec![]).await.unwrap();
    }
    if &addr[0..5] == "mysql" {
        info!("create mysql table if not exist");
        RB.exec(MYSQL_TABLE_USER, vec![]).await.unwrap();
        RB.exec(MYSQL_TABLE_BOOK, vec![]).await.unwrap();
        RB.exec(MYSQL_TABLE_BORROWED_BOOK, vec![]).await.unwrap();
        RB.exec(MYSQL_TABLE_RETURN_BOOK, vec![]).await.unwrap();
    }

    if exist("admin@admin.com").await.is_none() {
        info!("admin is not exist, create admin account");
        info!("email: admin@admin.com");
        info!("password: asdc1234ASD");

        let user = user::User {
            username: "admin".to_string(),
            password: "asdc1234ASD".to_string(),
            sid: "100000000000".to_string(),
            email: "admin@admin.com".to_string(),
            introduction: "admin".to_string(),
            age: "18".to_string(),
            sex: "unknown".to_string(),
            role: Role::Admin,
            status: user::Status::Enabled,
        };

        add(user, Some(Role::Admin)).await.unwrap();
    }
}
