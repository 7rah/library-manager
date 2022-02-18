use crate::api::JsonValue;
use poem::handler;
use poem::web::Json;

#[handler]
pub fn logout() -> JsonValue {
    Json(serde_json::json! ({
        "code": 20000,
        "data": "success",
    }))
}
