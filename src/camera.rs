use std::f32;
use nalgebra::Vector3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, view_up: Vector3<f32>, vertical_fov: f32, aspect: f32) -> Self {
        let theta = vertical_fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}