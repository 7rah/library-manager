pub mod info;
pub mod login;
pub mod logout;
pub mod register;
pub mod update;

lazy_static::lazy_static! {
    pub static ref RE_PASSWORD: regex::Regex = regex::Regex::new(r"^[a-z0-9A-Z](\.?[a-z0-9A-Z])*$").unwrap();
}
