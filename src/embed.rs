use poem::http::{header, Method, StatusCode};
use poem::{async_trait, Endpoint, Request, Response, Result};

#[derive(rust_embed::RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;
pub struct StaticEmbed;

#[async_trait]
impl Endpoint for StaticEmbed {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        if req.method() != Method::GET {
            return Ok(StatusCode::METHOD_NOT_ALLOWED.into());
        }

        let mut path = req
            .uri()
            .path()
            .trim_start_matches('/')
            .trim_end_matches('/')
            .to_string();
        if path.starts_with("dist/") {
            path = path.replace("dist/", "");
        } else if path.is_empty() {
            path = "index.html".to_string();
        }
        let path = path.as_ref();
        match Asset::get(path) {
            Some(content) => {
                let hash = hex::encode(content.metadata.sha256_hash());
                // if etag is matched, return 304
                if req
                    .headers()
                    .get(header::IF_NONE_MATCH)
                    .map(|etag| etag.to_str().unwrap_or("000000").eq(&hash))
                    .unwrap_or(false)
                {
                    return Ok(StatusCode::NOT_MODIFIED.into());
                }

                // otherwise, return 200 with etag hash
                let body: Vec<u8> = content.data.into();
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Ok(Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .header(header::ETAG, hash)
                    .body(body))
            }
            None => Ok(Response::builder().status(StatusCode::NOT_FOUND).finish()),
        }
    }
}
