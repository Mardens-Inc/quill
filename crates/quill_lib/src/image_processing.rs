use crate::print_orientation::PageOrientation;
use tracing::debug;

pub fn prepare_image(
    png: &image::RgbImage,
    orientation: PageOrientation,
    scale: f32,
) -> image::RgbImage {
    debug!(
        "Preparing image: source {}x{} px, scale {scale}, orientation {orientation:?}",
        png.width(),
        png.height()
    );
    let scaled = if scale > 0.0 && (scale - 1.0).abs() > f32::EPSILON {
        let w = ((png.width() as f32 * scale).round() as u32).max(1);
        let h = ((png.height() as f32 * scale).round() as u32).max(1);
        debug!("Resizing image to {w}x{h} px (scale {scale})");
        image::imageops::resize(png, w, h, image::imageops::FilterType::Lanczos3)
    } else {
        png.clone()
    };
    let rotated = rotate(scaled, orientation.degrees());
    debug!(
        "Prepared image: output {}x{} px",
        rotated.width(),
        rotated.height()
    );
    rotated
}

fn rotate(img: image::RgbImage, degrees: f32) -> image::RgbImage {
    let normalized = degrees.rem_euclid(360.0);
    let quarter = (normalized / 90.0).round();
    if (normalized - quarter * 90.0).abs() < 0.01 {
        match quarter as u32 % 4 {
            1 => image::imageops::rotate90(&img),
            2 => image::imageops::rotate180(&img),
            3 => image::imageops::rotate270(&img),
            _ => img,
        }
    } else {
        rotate_arbitrary(&img, normalized)
    }
}

fn rotate_arbitrary(img: &image::RgbImage, degrees: f32) -> image::RgbImage {
    debug!(
        "Rotating image {}x{} px by arbitrary angle {degrees}° (nearest-neighbour onto white canvas)",
        img.width(),
        img.height()
    );
    let theta = degrees.to_radians();
    let (sin, cos) = theta.sin_cos();
    let (w, h) = (img.width() as f32, img.height() as f32);

    let new_w = (w * cos.abs() + h * sin.abs()).ceil().max(1.0);
    let new_h = (w * sin.abs() + h * cos.abs()).ceil().max(1.0);
    let (out_w, out_h) = (new_w as u32, new_h as u32);

    let mut out = image::RgbImage::from_pixel(out_w, out_h, image::Rgb([255, 255, 255]));

    let (src_cx, src_cy) = (w / 2.0, h / 2.0);
    let (dst_cx, dst_cy) = (new_w / 2.0, new_h / 2.0);

    for y in 0..out_h {
        for x in 0..out_w {
            let dx = x as f32 + 0.5 - dst_cx;
            let dy = y as f32 + 0.5 - dst_cy;
            let sx = dx * cos + dy * sin + src_cx;
            let sy = -dx * sin + dy * cos + src_cy;

            if sx >= 0.0 && sy >= 0.0 {
                let (sxi, syi) = (sx as u32, sy as u32);
                if sxi < img.width() && syi < img.height() {
                    out.put_pixel(x, y, *img.get_pixel(sxi, syi));
                }
            }
        }
    }
    out
}
