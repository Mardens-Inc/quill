use crate::util::http_error::Result;
use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

#[get("/health")]
pub async fn get_health() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/version")]
pub async fn get_version() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(json!({ "version": env!("CARGO_PKG_VERSION").to_string(), "build": env!("BUILD").to_string() })))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("")
            .service(get_health)
            .service(get_version)
            .default_service(actix_web::web::to(|| async {
                HttpResponse::NotFound().json(json!({
                    "error": "API endpoint not found".to_string(),
                }))
            })),
    );
}
