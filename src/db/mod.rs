use log::info;
use rbatis::executor::Executor;
use rbatis::rbatis::Rbatis;

pub mod book;
pub mod record;
pub mod user;

lazy_static::lazy_static! {
    static ref RB:Rbatis=Rbatis::new();
}

pub fn encode_password(password: impl AsRef<str>) -> String {
    format!("{:x}", md5::compute(password.as_ref().as_bytes()))
}

pub async fn init_db(addr: &str) {
    info!("link db {addr}");
    RB.link(addr).await.unwrap();

    //table user
    RB.exec(
        "CREATE TABLE IF NOT EXISTS `user`(
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
     )ENGINE=InnoDB DEFAULT CHARSET=utf8;",
        vec![],
    )
    .await
    .unwrap();

    //table book
    RB.exec(
        "CREATE TABLE IF NOT EXISTS `book`(
        `name` VARCHAR(255),
        `author` VARCHAR(255),
        `isbn` VARCHAR(13),
        `press` VARCHAR(255),
        `remain` INT,
        `stock` INT,
        PRIMARY KEY ( `isbn` )
     )ENGINE=InnoDB DEFAULT CHARSET=utf8;",
        vec![],
    )
    .await
    .unwrap();

    //table borrowed book
    RB.exec(
        "CREATE TABLE IF NOT EXISTS `borrowed_book`(
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
     )ENGINE=InnoDB DEFAULT CHARSET=utf8;",
        vec![],
    )
    .await
    .unwrap();

    //table return book
    RB.exec(
        "CREATE TABLE IF NOT EXISTS `return_book`(
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
    )ENGINE=InnoDB DEFAULT CHARSET=utf8;",
        vec![],
    )
    .await
    .unwrap();
}
