use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self { Sphere {center, radius} }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal })
            }
        }
        None
    }
}