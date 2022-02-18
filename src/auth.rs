use crate::db::user::Role;
use crate::error::Error;
use jwt_simple::prelude::*;
use log::debug;

lazy_static::lazy_static! {
    static ref KEY:HS256Key = HS256Key::generate();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub email: String,
    pub role: Role,
}

impl Token {
    pub fn is_admin(&self) -> Option<()> {
        self.role.eq(&Role::Admin).then(|| ())
    }
}

pub fn create_token(email: impl AsRef<str>, role: Option<Role>) -> Result<String, Error> {
    let role = role.map_or(Role::User, |r| r);
    let token = Token {
        email: email.as_ref().to_string(),
        role,
    };
    let claims = Claims::with_custom_claims(token, Duration::from_hours(12));
    KEY.authenticate(claims).map_err(|e| {
        debug!("{e}");
        Error::FailedToCreateToken
    })
}

pub fn verify_token(token: impl AsRef<str>) -> Option<Token> {
    Some(KEY.verify_token::<Token>(token.as_ref(), None).ok()?.custom)
}
