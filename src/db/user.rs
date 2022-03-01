use super::RB;
use crate::error::Error;
use crate::types::{Age, Email, Introduction, Password, Role, Sex, Sid, Status, Username};
use log::debug;
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:user)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: Username,
    pub password: Password,
    pub sid: Sid,
    pub email: Email,
    pub introduction: Introduction,
    pub age: Age,
    pub sex: Sex,
    pub role: Role,
    pub status: Status,
}

#[crud_table(table_name:user)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<Username>,
    pub password: Option<Password>,
    pub sid: Option<Sid>,
    pub introduction: Option<Introduction>,
    pub age: Option<Age>,
    pub sex: Option<Sex>,
    pub role: Option<Role>,
    pub status: Option<Status>,
}

pub async fn exist(email: &Email) -> Option<()> {
    query(email).await.map(|_| ())
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

pub async fn query(email: &Email) -> Option<User> {
    RB.fetch_by_column::<Option<User>, _>("email", email)
        .await
        .ok()?
}

//default role: User
pub async fn add(mut user: User, role: Option<Role>) -> Option<()> {
    user.password = user.password.encode();
    let role = role.map_or(Role::User, |r| r);
    user.role = role;
    RB.save(&user, &[]).await.is_ok().then(|| ())
}

pub async fn verify(email: &Email, password: &Password) -> Option<User> {
    let user = query(email).await?;

    if user.password == password.encode() {
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

pub async fn update(email: &Email, mut user: UpdateUser) -> Result<(), Error> {
    exist(email).await.ok_or(Error::UserNotExist)?;
    if let Some(password) = user.password {
        user.password = Some(password.encode());
    }
    let w = RB.new_wrapper().eq("email", email);
    RB.update_by_wrapper(&user, w, &[Skip::Value(rbson::Bson::Null)])
        .await
        .map_err(|e| {
            debug!("{e}");
            Error::DbError
        })
        .map(|_| ())
}
