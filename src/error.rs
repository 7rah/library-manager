use poem::error::ResponseError;
use poem::http::StatusCode;
use serde::Deserialize;
use validator::ValidationErrors;

pub const SUCCESS_CODE: u32 = 20000;
fn fmt(code: u32, message: &str) -> String {
    format!("{{\"code\":{code},\"message\":\"{message}\"}}")
}

#[derive(Debug, thiserror::Error, Deserialize, Clone)]
pub enum Error {
    #[error("{}", fmt(40000, "邮箱或密码输入错误"))]
    InvalidEmailOrPassword,

    #[error("{}", fmt(50012, "无效的Token，请重新登录后重试"))]
    InvalidlToken,

    #[error("{}", fmt(50011, "Token 创建失败，请稍后再试"))]
    FailedToCreateToken,

    #[error("{}", fmt(30000, "用户已存在"))]
    UserAlreadyExist,

    #[error("{}", fmt(30001, "用户不存在"))]
    UserNotExist,

    #[error("{}", fmt(10000, "注册失败"))]
    FailedToRegister,

    #[error("{}", fmt(60000, "内部错误"))]
    InternalErr,

    #[error("{}", fmt(700000, "账号被禁用"))]
    AccountWasDisabled,

    #[error("{}", fmt(80000, "书籍已存在"))]
    BookAlreadyExist,

    #[error("{}", fmt(80001, "无法添加书籍"))]
    FailedToAddBook,

    #[error("{}", fmt(80002, "书籍不存在"))]
    BookNotExist,

    #[error("{}", fmt(80003, "删除书籍失败"))]
    FailedToDeleteBook,

    #[error("{}", fmt(90000, "无效的请求，请重新登录后重试"))]
    InvalidlRequest,

    #[error("{}", fmt(100000, "当前的用户不是管理员，无权操作"))]
    RoleNotAdmin,

    #[error("{}", fmt(110000, "书籍列表为空"))]
    BookListWasEmpty,

    #[error(
        "{{\"code\":{code},\"message\":\"{message} {0}\"}}",
        code = 120000,
        message = "不合法的数据，请检查你的输入"
    )]
    InvalidData(String),

    #[error("{}", fmt(130000, "非法的 ISBN 号"))]
    InvalidIsbn,

    #[error("{}", fmt(140000, "没有剩余书籍"))]
    NoRemainBook,

    #[error("{}", fmt(150000, "数据库错误"))]
    DbError,

    #[error("{}", fmt(160000, "你没有借过此书"))]
    BookIsNotBorrowed,

    #[error("{}", fmt(170000, "库存不足"))]
    StockIsntEnough,

    #[error("{}", fmt(180000, "错误的密码"))]
    InvalidPassword,
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        StatusCode::OK
    }
}

impl From<ValidationErrors> for Error {
    fn from(v: ValidationErrors) -> Self {
        Self::InvalidData(v.to_string())
    }
}
