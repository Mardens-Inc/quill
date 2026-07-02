use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use color_eyre::Result;
use quill_config::QuillSettings;
use serde_json::json;
use tracing::*;

mod health_endpoint;
mod util;
mod printer_endpoint;

pub static DEBUG: bool = cfg!(debug_assertions);
pub async fn run() -> Result<()> {
    util::logging::setup_logging()?;
    color_eyre::install()?;

    let port = QuillSettings::load()?.helper_service_port;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        let error = json!({ "error": format!("{}", err) });
                        actix_web::error::InternalError::from_response(
                            err,
                            HttpResponse::BadRequest().json(error),
                        )
                        .into()
                    }),
            )
            .service(web::scope("").configure(health_endpoint::configure))
    })
    .workers(4)
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run();

    info!(
        "Starting {} server at http://127.0.0.1:{}...",
        if DEBUG { "development" } else { "production" },
        port
    );

    let stop_result = server.await;
    debug!("Server stopped");

    Ok(stop_result?)
}
