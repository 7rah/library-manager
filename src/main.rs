use api::admin::user::list;
use api::book::list::get_list;
use api::book::return_record::list_return;
use api::user::info::get_info;
use api::user::login::login;
use api::user::logout::logout;
use api::user::register::register;
use backend::api::admin::book::{add_book, delete, update as update_book};
use backend::api::admin::user::update as update_user;
use backend::api::book::borrow::borrow_book;
use backend::api::book::borrow_record::list_borrow;
use backend::api::book::return_book::return_book;
use backend::api::book::search::search_list;
use backend::api::user::update::{change_password, update as update_user_info};
use backend::embed::StaticEmbed;
use backend::{api, init, middleware, CONFIG};
use log::info;
use middleware::{LogMiddleware, TokenMiddleware};
use poem::listener::TcpListener;
use poem::{get, post, EndpointExt, Result, Route, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init().await;

    let app = Route::new()
        .nest("/", StaticEmbed)
        .nest("/index.html", StaticEmbed)
        .nest("/prod-api/books-manager/user/register", post(register))
        .nest("/prod-api/books-manager/user/login", post(login))
        .nest("/prod-api/books-manager/user/info", get(get_info))
        .nest("/prod-api/books-manager/user/logout", get(logout))
        .nest(
            "/prod-api/books-manager/user/update",
            post(update_user_info),
        )
        .nest("/prod-api/books-manager/user/change_password",post(change_password))
        .nest("/prod-api/books-manager/book/search", post(search_list))
        .nest("/prod-api/books-manager/book/borrow", post(borrow_book))
        .nest("/prod-api/books-manager/book/return", post(return_book))
        .nest("/prod-api/books-manager/book/list", get(get_list))
        .nest("/prod-api/books-manager/book/list_borrow", get(list_borrow))
        .nest("/prod-api/books-manager/book/list_return", get(list_return))
        .nest("/prod-api/books-manager/admin/user/list", get(list))
        .nest("/prod-api/books-manager/admin/book/delete", post(delete))
        .nest("/prod-api/books-manager/admin/book/add", post(add_book))
        .nest(
            "/prod-api/books-manager/admin/book/update",
            post(update_book),
        )
        .nest(
            "/prod-api/books-manager/admin/user/update",
            post(update_user),
        )
        .with(LogMiddleware)
        .with(TokenMiddleware);

    let addr = &CONFIG.global.listen_addr;
    info!("serve at http://{addr}");
    Server::new(TcpListener::bind(addr)).run(app).await
}
