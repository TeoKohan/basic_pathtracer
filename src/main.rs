#[macro_export]
macro_rules! V3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3{x: $x, y: $y, z: $z}
    };
}

mod vector_3;
mod ray;
mod geometry;
mod hit;
mod camera;
mod colour;
mod material;
mod scene;

use geometry::sphere::Sphere;
use hit::{Surface, HitResult};
use xorshift::Rng;

use material::lambertian::Lambertian;
use crate::material::metallic::Metallic;
use crate::material::dielectric::Dielectric;

use crate::material::Material;

use crate::vector_3::{Vector3};
use crate::colour::Colour;
use crate::ray::Ray;

fn ray_colour(ray: &Ray, world: &dyn Surface, depth: u16, rng: &mut xorshift::StdRng) -> Colour {

    let hit_result: HitResult = world.hit(ray, 0.005, 1024.0);
    match hit_result {
        hit::HitResult::Hit(_, _, _, ref material, _) => {
            if depth > 0 {
                match material.scatter(ray, &hit_result, rng) {
                    material::Scatter::None => Colour::ZERO,
                    material::Scatter::Scatter(attenuation, scatter_ray) => {
                        attenuation * ray_colour(&scatter_ray, world, depth - 1, rng)
                    }
                }
            } else {
                Colour::ZERO
            }
        },
        hit::HitResult::None => {
            let unit_direction: Vector3 = ray.direction.unit_vector();
            let distance: f32 = 0.5 * (unit_direction.y + 1.0);
            (1.0 - distance) * Vector3::ONE + distance * V3!(0.5, 0.7, 1.0)
        }
    }
}

fn main() {

    //RANDOM
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: xorshift::StdRng = xorshift::SeedableRng::from_seed(seed);

    //IMAGE
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const WIDTH : u16 = 1200;
    const HEIGHT: u16 = (WIDTH as f32 / ASPECT_RATIO) as u16;
    const SAMPLES: u16 = 64;
    const DEPTH: u16 = 16;

    //WORLD
    let world: hit::HitList = scene::random_spheres::random_spheres_scene(&mut rng);

    //CAMERA
    let origin: Vector3 = V3!(13.0, 2.0, 3.0);
    let target: Vector3 = V3!(0.0, 0.0, 0.0);
    let camera: camera::Camera = camera::Camera::new(
        origin, 
        target, 
        Vector3::UP, 
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        );

    println!("P3\n{} {}", WIDTH, HEIGHT);
    println!("255");

    for j in (0..HEIGHT).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..WIDTH {
            let mut pixel_colour: Colour = Colour::ZERO;
            for _ in 0..SAMPLES {
                let u: f32 = (i as f32 + rng.next_f32()) / (WIDTH - 1) as f32;
                let v: f32 = (j as f32 + rng.next_f32()) / (HEIGHT- 1) as f32;
                let ray: Ray = camera.get_ray(u, v, &mut rng);
                pixel_colour += ray_colour(&ray, &world, DEPTH, &mut rng);
            }
            pixel_colour = pixel_colour / SAMPLES as f32;
            colour::write_colour(&pixel_colour);
        }
    }
    eprintln!("done.");
}
