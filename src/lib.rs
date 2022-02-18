pub mod api;
pub mod auth;
pub mod config;
pub mod db;
pub mod embed;
pub mod error;
pub mod middleware;

lazy_static::lazy_static! {
    pub static ref CONFIG:config::Config =  {
        config::init_config()
    };
}

pub async fn init() {
    middleware::init_log(&CONFIG.global.log);
    db::init_db(&CONFIG.db.addr).await;
}
