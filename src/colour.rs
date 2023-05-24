use std::f32::consts::PI;
use crate::vector_3::Vector3;

pub type Colour = Vector3;

pub fn hsv2rgb(hsv: &Colour) -> Colour {

    let mut hh: f32 = hsv.x;
    hh = hh / PI * 3.0;

    let i: u32 = hh as u32;
    let ff: f32 = hh - i as f32;
    let p: f32 = hsv.z * (1.0 - hsv.y);
    let q: f32 = hsv.z * (1.0 - (hsv.y * ff));
    let t: f32 = hsv.z * (1.0 - (hsv.y * (1.0 - ff)));

    match i {
    0 => { V3!(hsv.z, t, p) },
    1 => { V3!(q, hsv.z, p) },
    2 => { V3!(p, hsv.z, t) },
    3 => { V3!(p, q, hsv.z) },
    4 => { V3!(t, p, hsv.z) },
    _ => { V3!(hsv.z, p, q) }
    } 
}

pub fn write_colour(pixel_colour: &Colour) -> () {

    let pixel_colour: Colour = *pixel_colour;
    println!("{} {} {}",
        (256.0 * pixel_colour.x.sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * pixel_colour.y.sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * pixel_colour.z.sqrt().clamp(0.0, 0.999)) as u8);
}