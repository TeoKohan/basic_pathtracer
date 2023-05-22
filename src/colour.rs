use crate::vector_3;
pub type Colour = vector_3::Vector3;

pub fn write_colour(pixel_colour: &Colour) -> () {

    let pixel_colour: Colour = *pixel_colour;
    println!("{} {} {}",
        (256.0 * pixel_colour.x.sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * pixel_colour.y.sqrt().clamp(0.0, 0.999)) as u8,
        (256.0 * pixel_colour.z.sqrt().clamp(0.0, 0.999)) as u8);
}