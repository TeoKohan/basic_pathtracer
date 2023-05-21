use crate::vector_3;
pub type Colour = vector_3::Vector3;

pub fn write_colour(pixel_colour: &Colour) -> () {

    let pixel_colour: Colour = *pixel_colour;
    println!("{} {} {}",
        (255.999 * pixel_colour.x) as u8,
        (255.999 * pixel_colour.y) as u8,
        (255.999 * pixel_colour.z) as u8);
}