use image::{DynamicImage, RgbaImage};
use ravif::{Encoder, Img, RGBA8};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use savvy::{savvy, IntegerSexp, NumericScalar, OwnedLogicalSexp, OwnedRawSexp};
use viuer::{print, Config};

/// Encodes an AVIF image
///
/// @param nr The image to encode
/// @param height The height of the image
/// @param width The width of the image
/// @param quarlity The quality of the image
/// @returns The encoded image
/// @noRd
#[savvy]
fn encode_avif(
    nr: IntegerSexp,
    height: NumericScalar,
    width: NumericScalar,
    quality: NumericScalar,
    speed: NumericScalar,
) -> savvy::Result<savvy::Sexp> {
    let height = height.as_f64() as usize;
    let width = width.as_f64() as usize;
    let quality = quality.as_f64() as f32;
    let speed = speed.as_f64() as u8;

    let nr: Vec<RGBA8> = nr
        .to_vec()
        .par_iter()
        .map(|px| {
            let px = *px as u32;
            let a = (px >> 24) as u8;
            let b = ((px >> 16) & 0xff) as u8;
            let g = ((px >> 8) & 0xff) as u8;
            let r = (px & 0xff) as u8;
            RGBA8 { r, g, b, a }
        })
        .collect();

    let enc = Encoder::new().with_quality(quality).with_speed(speed);

    let img = enc.encode_rgba(Img::new(nr.as_slice(), width, height))?;

    let buf = OwnedRawSexp::try_from(img.avif_file)?;
    Ok(buf.into())
}

/// Print a native raster image to the terminal
///
/// @param nr a native raster image
/// @param width width
/// @param height height
/// @param term_cols terminal width
/// @noRd
#[savvy]
fn print_with_viuer(
    nr: IntegerSexp,
    height: i32,
    width: i32,
    term_cols: i32,
) -> savvy::Result<savvy::Sexp> {
    let nr = nr.to_vec();

    let rgba = RgbaImage::from_fn(width as u32, height as u32, |x, y| {
        let px = nr[x as usize + y as usize * width as usize] as u32;
        let a = (px >> 24) as u8;
        let b = ((px >> 16) & 0xff) as u8;
        let g = ((px >> 8) & 0xff) as u8;
        let r = (px & 0xff) as u8;

        image::Rgba([r, g, b, a])
    });

    let img = DynamicImage::ImageRgba8(rgba);

    let conf = Config {
        width: Some(term_cols as u32),
        ..Default::default()
    };

    print(&img, &conf)?;

    Ok(OwnedLogicalSexp::try_from_scalar(true)?.into())
}
