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


use geometry::sphere::Sphere;
use hit::{Surface, HitResult};
use xorshift::Rng;
use std::rc::Rc;
use material::lambertian::Lambertian;

use crate::material::Material;
use crate::material::metallic::Metallic;
use crate::vector_3::{Vector3, Point3};
use crate::colour::Colour;
use crate::ray::Ray;

fn ray_colour(ray : &Ray, world : &dyn Surface, depth: u16, rng: &mut xorshift::StdRng) -> Colour {

    let hit_result: HitResult = world.hit(ray, 0.005, 1024.0);
    match hit_result {
        hit::HitResult::Hit(position, normal, _, ref material, _) => {
            let target: Point3 = position + normal + Vector3::random_unit_vector(rng);
            if depth > 0 {
                match material.scatter(ray, &hit_result, rng) {
                    material::Scatter::None => Colour::ZERO,
                    material::Scatter::Scatter(attenuation, scatter_ray) => {
                        attenuation.abc(&ray_colour(&scatter_ray, world, depth - 1, rng))
                    }
                }
                //0.5 * ray_colour(&Ray{origin: position, direction: target - position}, world, depth - 1, rng)
            } else {
                Colour::ZERO
            }
        },
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
    const WIDTH  : u16 = 960;
    const HEIGHT : u16 = (WIDTH as f32 / ASPECT_RATIO) as u16;
    const SAMPLES : u16 = 100;
    const DEPTH : u16 = 32;

    //MATERIALS
    let material_grey: Rc::<dyn Material> = Rc::new(Lambertian{ albedo: 0.7 * Vector3::ONE });
    let material_red: Rc::<dyn Material> = Rc::new(Lambertian{ albedo: V3!(0.8, 0.2, 0.1) });
    let material_blue: Rc::<dyn Material> = Rc::new(Lambertian{ albedo: V3!(0.1, 0.2, 0.8) });
    let material_yellow: Rc::<dyn Material> = Rc::new(Lambertian{ albedo: V3!(0.85, 0.7, 0.1) });
    let material_metal: Rc::<dyn Material> = Rc::new(Metallic{ albedo: 0.85 * Vector3::ONE });
    

    //WORLD
    let mut world: hit::HitList = hit::HitList{ objects: vec![ ]};
    world.add(Sphere{center: V3!(-0.25, 0.00, -1.50), radius: 0.5, material: material_red.clone()});
    world.add(Sphere{center: V3!( 0.25, 0.00, -1.00), radius: 0.5, material: material_blue.clone()});
    world.add(Sphere{center: V3!(-0.75, 0.00, -1.75), radius: 0.5, material: material_yellow.clone()});

    world.add(Sphere{center: V3!( 1.25, 0.00, -1.00), radius: 0.5, material: material_metal.clone()});
    world.add(Sphere{center: V3!(-1.25, 0.00, -1.00), radius: 0.5, material: material_metal.clone()});

    world.add(Sphere{center: V3!( 0.00, -100.5, -1.00), radius: 100.0, material: material_grey.clone()});

    //CAMERA
    let viewport_height : f32 = 2.0;
    let viewport_width : f32 = ASPECT_RATIO * viewport_height;
    let camera: camera::Camera = camera::Camera {
        origin: V3!(0.0, 0.0, 0.0),
        horizontal: viewport_width * Vector3::X,
        vertical: viewport_height * Vector3::Y,
        focal_length: 0.8
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
                pixel_colour += ray_colour(&ray, &world, DEPTH, &mut rng);
            }
            pixel_colour = pixel_colour / SAMPLES as f32;
            colour::write_colour(&pixel_colour);
        }
    }
    eprintln!("done.");
}
