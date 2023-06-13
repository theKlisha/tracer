use crate::math::{Point3, Vector3};

pub mod material;
pub mod simple_renderer;

#[derive(Debug)]
pub struct SurfaceInteraction {
    pub point: Point3,
    pub direction: Vector3,
    pub normal: Vector3,
    pub face: Face,
    pub t: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Face {
    Front,
    Back,
}

impl SurfaceInteraction {
    pub fn new(point: Point3, direction: Vector3, normal: Vector3, t: f64) -> Self {
        let face = match normal.dot(&direction) > 0.0 {
            true => Face::Front,
            false => Face::Back,
        };

        Self {
            point,
            direction,
            normal,
            face,
            t,
        }
    }
}
