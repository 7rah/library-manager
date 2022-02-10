use backend::embed::StaticEmbed;
use backend::logging::Log;
use log::{error, info, warn};
use poem::{
    async_trait, get, handler, Endpoint, EndpointExt, IntoResponse, Middleware, Request, Response,
    Result,
};
use poem::{listener::TcpListener, Route, Server};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(Level::INFO)
        .init();

    let app = Route::new()
        .nest("/", StaticEmbed)
        .nest("/index.html", StaticEmbed)
        .with(Log);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
