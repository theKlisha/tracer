use std::{
    iter::Sum,
    ops::{Add, Deref, Mul},
};

use image::Rgba;
use nalgebra;

pub type Point2 = nalgebra::Point2<f64>;
pub type Point3 = nalgebra::Point3<f64>;
pub type Vector2 = nalgebra::Vector2<f64>;
pub type Vector3 = nalgebra::Vector3<f64>;
pub type RowVector3 = nalgebra::RowVector3<f64>;
pub type Matrix3 = nalgebra::Matrix3<f64>;
pub type Matrix4 = nalgebra::Matrix4<f64>;
pub type Matrix4x3 = nalgebra::Matrix4x3<f64>;

#[derive(Debug, Clone)]
pub struct Color(Vector3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vector3::new(r, g, b))
    }
}

impl Deref for Color {
    type Target = Vector3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0.component_mul(&rhs.0))
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

impl Color {
    pub fn sigmoid_exposure(&self) -> Rgba<u8> {
        todo!()
    }

    pub fn clip_exposure(self, gamma: f64) -> Rgba<u8> {
        Rgba([
            (self.0.x.min(1.0).max(0.0).powf(gamma) * 255.0) as u8,
            (self.0.y.min(1.0).max(0.0).powf(gamma) * 255.0) as u8,
            (self.0.z.min(1.0).max(0.0).powf(gamma) * 255.0) as u8,
            255,
        ])
    }
}
