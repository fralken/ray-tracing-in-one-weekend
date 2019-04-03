mod ray;
mod hitable;
mod sphere;
mod camera;

use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitableList};
use crate::sphere::Sphere;
use crate::camera::Camera;

fn color(ray: &Ray, world: &HitableList) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.0, f32::MAX) {
        0.5 * hit.normal.add_scalar(1.0)
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0))
    ]);
    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world);
            }
            col /= ns as f32;
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
