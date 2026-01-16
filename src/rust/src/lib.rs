use ravif::{Encoder, Img, RGBA8};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use savvy::{savvy, IntegerSexp, NumericScalar, OwnedRawSexp};

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
