use crate::auth::{verify_token, Token};
use crate::db::user::query;
use crate::error::Error;
use crate::types::Status;
use log::debug;
use poem::{Endpoint, IntoResponse, Request, Response, Result};

const TOKEN_HEADER: &str = "X-Token";

async fn is_user_enabled(token: &Token) -> bool {
    query(&token.email)
        .await
        .and_then(|user| user.status.ne(&Status::Disabled).then(|| ()))
        .is_some()
}

pub async fn token<E: Endpoint>(next: E, mut req: Request) -> Result<Response> {
    if let Some(value) = req
        .headers()
        .get(TOKEN_HEADER)
        .and_then(|value| value.to_str().ok())
    {
        let token = verify_token(value);
        debug!("Token: {token:?}");

        if let Some(token) = token {
            if is_user_enabled(&token).await {
                req.extensions_mut().insert(token);
            } else {
                return Ok(Error::AccountWasDisabled.to_string().into_response());
            }
        } else {
            return Ok(Error::InvalidlToken.to_string().into_response());
        }
    }

    // call the inner endpoint.
    next.call(req).await.map(IntoResponse::into_response)
}
