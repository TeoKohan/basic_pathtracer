#[macro_export]
macro_rules! V3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3{x: $x, y: $y, z: $z}
    };
}

mod random;
mod vector_3;
mod colour;
mod ray;
mod geometry;
mod hit;
mod camera;

use geometry::sphere::Sphere;
use hit::Surface;
use xorshift::Rng;

use crate::vector_3::{Vector3, Point3};
use crate::colour::Colour;
use crate::ray::Ray;

fn ray_colour(ray : &Ray, world : &dyn Surface) -> Colour {
    let hit_result: hit::HitResult = world.hit(ray, 0.0, 1024.0);
    match hit_result {
        hit::HitResult::Hit(_, normal, _, _) => 0.5 * (normal.unit_vector() + Vector3::ONE),
        hit::HitResult::None => {
            let unit_direction : Vector3 = ray.direction.unit_vector();
            let t : f32 = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector3::ONE + t * V3!(0.5, 0.7, 1.0)
        }
    }
}

fn main() {

    //RANDOM
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: xorshift::StdRng = xorshift::SeedableRng::from_seed(seed);

    //IMAGE
    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const WIDTH  : u16 = 400;
    const HEIGHT : u16 = (WIDTH as f32 / ASPECT_RATIO) as u16;
    const SAMPLES : u16 = 100;

    //WORLD
    let mut world: hit::HitList = hit::HitList{ objects: vec![ ]};
    world.add(Sphere{center: -Vector3::Z, radius: 0.5});
    world.add(Sphere{center: V3!(0.0, -100.5, -1.0), radius: 100.0});

    //CAMERA
    let viewport_height : f32 = 2.0;
    let viewport_width : f32 = ASPECT_RATIO * viewport_height;
    let camera: camera::Camera = camera::Camera {
        origin: Vector3::ZERO,
        horizontal: viewport_width * Vector3::X,
        vertical: viewport_height * Vector3::Y,
        focal_length: 1.0
    };

    println!("P3\n{} {}", WIDTH, HEIGHT);
    println!("255");

    for j in (0..HEIGHT).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..WIDTH {
            let mut pixel_colour : Colour = Colour::ZERO;
            for _ in 0..SAMPLES {
                let u : f32 = (i as f32 + rng.next_f32()) / (WIDTH - 1) as f32;
                let v : f32 = (j as f32 + rng.next_f32()) / (HEIGHT- 1) as f32;
                let ray : Ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world);
            }
            pixel_colour = pixel_colour / SAMPLES as f32;
            colour::write_colour(&pixel_colour);
        }
    }
    eprintln!("done.");
}
