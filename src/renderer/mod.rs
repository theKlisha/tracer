use crate::math::{Point3, Vector3};

pub mod simple_renderer;

#[derive(Debug)]
pub struct HitRecord {
    point: Point3,
    direction: Vector3,
    normal: Vector3,
    face: Option<Face>,
    t: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Face {
    Front,
    Back,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        direction: Vector3,
        surface_normal: Vector3,
        t: f64,
    ) -> Self {
        Self {
            point,
            direction,
            normal: surface_normal,
            face: None,
            t,
        }
    }

    pub fn face(&mut self) -> Face {
        match self.face {
            Some(face) => face,
            None => {
                let face = match self.normal.dot(&self.direction) > 0.0 {
                    true => Face::Front,
                    false => Face::Back,
                };

                self.face = Some(face);

                face
            }
        }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}


