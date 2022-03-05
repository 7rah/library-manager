use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;
use validator::{
    validate_email, validate_length, validate_range, Validate, ValidationError, ValidationErrors,
};

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Username(String);

fn new_err(field: &'static str, s: &'static str) -> ValidationErrors {
    let mut v = ValidationErrors::new();
    v.add(field, ValidationError::new(s));
    v
}

impl Validate for Username {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_length(&self.0, Some(1), Some(10), None)
            .then(|| ())
            .ok_or_else(|| new_err("username", "姓名为 10 个以下中英文字符"))
    }
}

impl From<&str> for Username {
    fn from(s: &str) -> Self {
        Username(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bookname(String);

impl Validate for Bookname {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_length(&self.0, Some(1), Some(50), None)
            .then(|| ())
            .ok_or_else(|| new_err("bookname", "书籍名称为 50 以下中英文字符"))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Introduction(String);

impl Validate for Introduction {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_length(&self.0, Some(0), Some(200), None)
            .then(|| ())
            .ok_or_else(|| new_err("introduction", "自我介绍为 200 以下中英文字符"))
    }
}

impl From<&str> for Introduction {
    fn from(s: &str) -> Self {
        Introduction(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email(String);

impl Validate for Email {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_email(&self.0)
            .then(|| ())
            .ok_or_else(|| new_err("introduction", "邮箱格式不正确"))
    }
}

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sid(String);

impl Validate for Sid {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let e = new_err("studentid", "学号为 12 位纯数字");
        if self.0.len() != 12 {
            return Err(e);
        }

        for c in self.0.chars() {
            if !c.is_ascii_digit() {
                return Err(e);
            }
        }

        Ok(())
    }
}

impl From<&str> for Sid {
    fn from(s: &str) -> Self {
        Sid(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Isbn(String);

impl Validate for Isbn {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let e = new_err("isbn", "ISBN 号为 13 位纯数字");
        if self.0.len() != 13 {
            return Err(e);
        }

        for c in self.0.chars() {
            if !c.is_ascii_digit() {
                return Err(e);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author(String);

impl Validate for Author {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_length(&self.0, Some(1), Some(20), None)
            .then(|| ())
            .ok_or_else(|| new_err("author", "作者名为 20 以下中英文字符"))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Press(String);

impl Validate for Press {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_length(&self.0, Some(1), Some(20), None)
            .then(|| ())
            .ok_or_else(|| new_err("press", "出版社为 20 以下中英文字符"))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Stock(u32);

impl Validate for Stock {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_range(&self.0, Some(&0u32), Some(&100u32))
            .then(|| ())
            .ok_or_else(|| new_err("stock", "书籍库存为 0~100 之间整数"))
    }
}

impl From<u32> for Stock {
    fn from(n: u32) -> Self {
        Stock(n)
    }
}

impl Stock {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

lazy_static::lazy_static! {
 static ref RE_PASSWORD: regex::Regex = regex::Regex::new(r"^[a-z0-9A-Z](\.?[a-z0-9A-Z])*$").unwrap();
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Validate for Password {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        RE_PASSWORD.is_match(&self.0).then(|| ()).ok_or_else(|| {
            new_err(
                "password",
                "密码为 8~16 位大小写字母加数字的组合（不包含特殊字符）",
            )
        })
    }
}

impl Password {
    pub fn encode(&self) -> Password {
        Password(format!("{:x}", md5::compute(self.0.as_bytes())))
    }
}

impl From<&str> for Password {
    fn from(s: &str) -> Self {
        Password(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Age(String);

impl Validate for Age {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let age = self
            .0
            .parse()
            .map_err(|_| new_err("age", "falied to parse int"))?;
        validate_range(&age, Some(&0u32), Some(&100u32))
            .then(|| ())
            .ok_or_else(|| new_err("age", "年龄为 0~100 间的整数"))
    }
}

impl From<u32> for Age {
    fn from(n: u32) -> Self {
        Age(n.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sex(String);

impl Validate for Sex {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self.0.as_str() {
            "male" => Ok(()),
            "female" => Ok(()),
            "unknown" => Ok(()),
            _ => Err(new_err("sex", "expected `male` `female` `unknown`")),
        }
    }
}

impl From<&str> for Sex {
    fn from(s: &str) -> Self {
        Sex(s.to_string())
    }
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Role {
    Admin = 0,
    User = 1,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Disabled = 0,
    Enabled = 1,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Admin => "admin".to_string(),
            Role::User => "user".to_string(),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Enabled => "enabled".to_string(),
            Status::Disabled => "disabled".to_string(),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enabled" => Ok(Status::Enabled),
            "disabled" => Ok(Status::Disabled),
            _ => Err(Error::InvalidData(
                "unknown status, excepted `enabled` or `disabled`".into(),
            )),
        }
    }
}
