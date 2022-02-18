use std::str::FromStr;

use super::{encode_password, RB};
use crate::error::Error;
use log::debug;
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use serde::{Deserialize, Serialize};
use serde_repr::*;

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
            _ => Err(Error::InvalidData),
        }
    }
}

#[crud_table(table_name:user)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub sid: String,
    pub email: String,
    pub introduction: String,
    pub age: String,
    pub sex: String,
    pub role: Role,
    pub status: Status,
}

#[crud_table(table_name:user)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub sid: Option<String>,
    pub introduction: Option<String>,
    pub age: Option<String>,
    pub sex: Option<String>,
    pub role: Option<Role>,
    pub status: Option<Status>,
}

pub async fn exist(email: impl AsRef<str>) -> Result<(), Error> {
    query(email.as_ref())
        .await
        .ok_or(Error::UserAlreadyExist)
        .map(|_| ())
    /*
    let w = RB.new_wrapper().eq("email", email.as_ref());
    let c = RB
        .fetch_count_by_wrapper::<Option<User>>(w)
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::InternalErr
        })?;
    if c != 0 {
        Ok(())
    } else {
        Err(Error::UserNotExist)
    }
    */
}

pub async fn query(email: impl AsRef<str>) -> Option<User> {
    RB.fetch_by_column::<Option<User>, _>("email", email.as_ref())
        .await
        .ok()?
}

//default role: User
pub async fn add(mut user: User, role: Option<Role>) -> Option<()> {
    let role = role.map_or(Role::User, |r| r);
    user.role = role;
    RB.save(&user, &[]).await.is_ok().then(|| ())
}

pub async fn verify(email: impl AsRef<str>, password: impl AsRef<str>) -> Option<User> {
    let user = query(email).await?;
    if user.password == encode_password(password) {
        Some(user)
    } else {
        None
    }
}

pub async fn list() -> Result<Vec<User>, Error> {
    RB.fetch_list::<User>().await.map_err(|e| {
        debug!("{e}");
        Error::DbError
    })
}

pub async fn update(email: impl AsRef<str>,mut user: UpdateUser) -> Result<(), Error> {
    exist(email.as_ref()).await?;
    if let Some(password) = user.password {
        user.password = Some(encode_password(&password));
    }
    let w = RB.new_wrapper().eq("email", email.as_ref());
    RB.update_by_wrapper(&user, w, &[Skip::Value(rbson::Bson::Null)])
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })
        .map(|_| ())
}
