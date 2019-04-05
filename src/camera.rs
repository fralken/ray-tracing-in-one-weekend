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
    pub fn new(vertical_fov: f32, aspect: f32) -> Self {
        let theta = vertical_fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(-half_width, -half_height, -1.0),
            horizontal: Vector3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * half_height, 0.0)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}