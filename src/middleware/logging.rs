use crate::CONFIG;
use colored::Colorize;
use log::{info, warn};
use poem::{Endpoint, IntoResponse, Request, Response, Result};
use time::{format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub fn init_log(level: &str) {
    let mut filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap()
        .add_directive("poem::server=warn".parse().unwrap())
        .add_directive("hyper::proto::h1=warn".parse().unwrap());
    if let Some(level) = &CONFIG.db.log {
        filter = filter.add_directive(format!("rbatis={level}").parse().unwrap());
    }
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
        )
        .unwrap(),
    );
    let fmt = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(local_time);
    tracing_subscriber::registry().with(filter).with(fmt).init();
}

pub async fn log<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    let uri = String::from(req.uri().path());
    let method = req.method().clone();
    let method = if uri.contains("prod-api") {
        method.to_string().blue().to_string()
    } else {
        method.to_string()
    };
    let res = next.call(req).await;

    match res {
        Ok(resp) => {
            let resp = resp.into_response();
            let stat = resp.status();
            info!("[{stat}] {method} {uri}");
            Ok(resp)
        }
        Err(err) => {
            warn!("[{err}] {method} {uri}");

            Err(err)
        }
    }
}
