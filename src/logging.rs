use log::{error, info, warn};
use poem::{
    async_trait, get, handler, Endpoint, EndpointExt, IntoResponse, Middleware, Request, Response,
    Result,
};
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use tracing::Level;
use tracing_subscriber::fmt::SubscriberBuilder;
use tracing_subscriber::fmt::writer::MakeWriterExt;


pub struct Log;

impl<E: Endpoint> Middleware<E> for Log {
    type Output = LogImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogImpl(ep)
    }
}

pub struct LogImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for LogImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let uri = String::from(req.uri().path());
        let method = req.method().clone();
        //info!("{method} {uri}");
        let res = self.0.call(req).await;

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
}
