use crate::auth::Token;
use crate::db::user::Role;
use crate::error::Error;

pub mod book;
pub mod user;

fn is_admin(token: &Token) -> Result<(), Error> {
    token
        .role
        .eq(&Role::Admin)
        .then(|| ())
        .ok_or(Error::RoleNotAdmin)
}
