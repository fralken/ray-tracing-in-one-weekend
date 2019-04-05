mod ray;
mod hitable;
mod material;
mod sphere;
mod camera;

use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::hitable::{Hitable, HitableList};
use crate::sphere::Sphere;
use crate::camera::Camera;

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HitableList::default();
    world.push(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Vector3::new(0.5, 0.5, 0.5))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 { // diffuse
                    world.push(
                        Sphere::new(center, 0.2,
                                    Lambertian::new(Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>()))));
                } else if choose_material < 0.95 { // metal
                    world.push(
                        Sphere::new(center, 0.2,
                                    Metal::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>())));
                } else { // glass
                    world.push( Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vector3::new(0.4, 0.2, 0.1))));
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)));
    world
}

fn color(ray: &Ray, world: &HitableList, depth: i32) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                return attenuation.zip_map(&color(&scattered, &world, depth+1), |l, r| l * r);
            }
        }
        Vector3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    println!("P3\n{} {}\n255", nx, ny);
    let world = random_scene();
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_at, Vector3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32, aperture, focus_dist);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;
            for c in col.iter_mut() { *c = c.sqrt(); }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
