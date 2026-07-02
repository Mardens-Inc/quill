use crate::util::http_error::Result;
use actix_web::{HttpResponse, Responder, post, web};
use image::{RgbImage, load_from_memory};
use quill_config::QuillSettings;
use quill_lib::print_orientation::PageOrientation;
use quill_lib::printers::Printers;
use quill_lib::stock::Stock;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
struct PrintParameters {
    stock_id: String,
}

#[post("/print")]
pub async fn print(
    params: web::Query<PrintParameters>,
    body: web::Bytes,
) -> Result<impl Responder> {
    let settings = QuillSettings::load()?;

    let printer = match settings.selected_printer {
        Some(printer) => printer,
        None => {
            return Ok(HttpResponse::BadRequest().json(json!({"message":"No printer selected"})));
        }
    };
    let stock: Stock = match settings
        .labels
        .iter()
        .find(|stock| stock.id == params.stock_id)
    {
        Some(label) => label.into(),
        None => return Ok(HttpResponse::NotFound().json(json!({"message": "No Stock found"}))),
    };
    let orientation = match settings.default_orientation {
        0 => PageOrientation::Normal,
        1 => PageOrientation::Rotate90,
        2 => PageOrientation::Rotate180,
        3 => PageOrientation::Rotate270,
        val => PageOrientation::Degrees(val as f32),
    };
    let handle = match Printers::get_printer_handle(printer) {
        Ok(printer) => printer,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .json(json!({"message": "Failed to get printer handle"})));
        }
    };
    let uuid = Uuid::new_v4();
    let bytes = body.to_vec();
    let bytes = bytes.as_slice();
    let png: RgbImage = match load_from_memory(bytes) {
        Ok(image) => image,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .json(json! {{"message": "Failed to load image"}}));
        }
    }
    .to_rgb8();
    if handle
        .print_png(
            format!("price_tagger-{uuid}"),
            &png,
            stock,
            orientation,
            settings.scale as f32 / 100f32,
            settings.monochrome_threshold as u32,
            settings.density as f32,
        )
        .is_err()
    {
        return Ok(HttpResponse::InternalServerError()
            .json(json! {{"message": "Failed to create print job"}}));
    }

    Ok(HttpResponse::Ok().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/printers").service(print))
        .default_service(web::to(|| async {
            HttpResponse::NotFound().json(json!({
                "error": "API endpoint not found".to_string(),
            }))
        }));
}
