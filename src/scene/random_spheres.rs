use std::f32::consts::PI;
use std::rc::Rc;
use xorshift::Rng;

use crate::colour;
use crate::hit::HitList;
use crate::Sphere;
use crate::Material;
use crate::{Lambertian, Metallic, Dielectric};
use crate::{Vector3, vector_3::Point3};
use crate::colour::Colour;

pub fn random_spheres_scene(rng: &mut xorshift::StdRng) -> HitList {

    struct CollisionSphere {
        center: Point3,
        radius: f32
    }

    let mut world: HitList = HitList{ objects: vec![ ]};
    let mut spheres: Vec<CollisionSphere> = vec![];
    let radius: f32 = 1.0;

    let material: Rc::<dyn Material> = Rc::new(Lambertian{albedo: 0.5 * Colour::ONE});
    world.add(Sphere{center: V3!(0.0, -1000.0, 0.0), radius: 1000.0, material});

    let material: Rc::<dyn Material> = Rc::new(Dielectric{refraction_index: 1.5});
    world.add(Sphere{center: V3!(0.0, 1.0, 0.0), radius, material});
    spheres.push(CollisionSphere{center: V3!(0.0, 1.0, 0.0), radius });
    
    let material: Rc::<dyn Material> = Rc::new(Lambertian{albedo: V3!(0.4, 0.2, 0.1)});
    world.add(Sphere{center: V3!(-4.0, 1.0, 0.0), radius, material});
    spheres.push(CollisionSphere{center: V3!(-4.0, 1.0, 0.0), radius });

    let material: Rc::<dyn Material> = Rc::new(Metallic{albedo: V3!(0.7, 0.6, 0.5), fuzziness: 0.0});
    world.add(Sphere{center: V3!(4.0, 1.0, 0.0), radius, material});
    spheres.push(CollisionSphere{center: V3!(4.0, 1.0, 0.0), radius });

    const N: i16 = 25;

    for i in -N..N {
        for j in -N..N {
            let radius: f32 = 0.2;

            let choose_mat: f32 = rng.next_f32();
            let mut center: Point3;
            
            loop {
                center = V3!(i as f32 + 0.9 * rng.next_f32(), 0.2, j as f32 + 0.9 * rng.next_f32());
                
                fn sphere_intersection(a: &CollisionSphere, b: &CollisionSphere) -> bool {
                    Vector3::length(&(a.center - b.center)) < a.radius + b.radius + 0.05
                }
                
                if spheres.iter().all(|b: &CollisionSphere| !sphere_intersection(&CollisionSphere {center, radius}, &b)) {
                    break;
                };
            }

            if (center - V3!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let hsv: Colour = Colour{x: rng.next_f32() * 2.0 * PI, y: 1.0, z: 0.8};
                    let albedo: Colour = colour::hsv2rgb(&hsv);
                    let material: Rc::<dyn Material> = Rc::new(Lambertian{albedo});
                    world.add(Sphere{center, radius: 0.2, material});
                } else if choose_mat < 0.95 {
                    // metal
                    let hsv: Colour = Colour{x: rng.next_f32() * 2.0 * PI, y: 0.2, z: 0.8};
                    let albedo: Colour = colour::hsv2rgb(&hsv);
                    let fuzziness: f32 = rng.next_f32() / 2.0;
                    let material: Rc::<dyn Material> = Rc::new(Metallic{albedo, fuzziness});
                    world.add(Sphere{center, radius: 0.2, material});
                } else {
                    // glass
                    let material: Rc::<dyn Material> = Rc::new(Dielectric{refraction_index: 1.5});
                    world.add(Sphere{center, radius: 0.2, material});
                }
            }
        }
    }

    return world;
}