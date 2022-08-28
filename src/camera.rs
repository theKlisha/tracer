use crate::Ray;
use derive_builder::Builder;
use nalgebra::{Matrix3, Matrix4, Matrix4x3, Point3, RowVector3, Vector3};

pub trait Camera {
    fn ray_caster(&self) -> RayCaster;
}

pub struct RayCaster {
    origin: Vector3<f32>,
    transform: Matrix4x3<f32>,
}

impl RayCaster {
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.transform * Vector3::new(u, v, 1.0);
        let direction = Vector3::from_homogeneous(direction).unwrap().normalize();
        Ray::new(self.origin, direction)
    }
}

#[derive(Builder)]
pub struct PerspectiveCamera {
    image_width: u32,
    image_height: u32,
    focal_lenght: f32,
    pixel_width: f32,
    pixel_height: f32,
    translation: Vector3<f32>,
    rotation_matrix: Matrix4<f32>,
}

impl PerspectiveCameraBuilder {
    pub fn image_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.image_width = Some(width);
        self.image_height = Some(height);
        self
    }

    pub fn look_at(&mut self, target: Point3<f32>) -> &mut Self {
        let matrix = Matrix4::look_at_rh(
            &self.translation.unwrap().into(),
            &target,
            &Vector3::new(0.0, 1.0, 0.0),
        );

        self.rotation_matrix(matrix);
        self
    }
}

impl Camera for PerspectiveCamera {
    fn ray_caster(&self) -> RayCaster {
        let extrinsics = {
            let translation_inv = Matrix4::new_translation(&self.translation)
                .try_inverse()
                .unwrap();
            let rotation_inv = self.rotation_matrix.try_inverse().unwrap();
            translation_inv * rotation_inv
        };

        let intrinsics = {
            let f = self.focal_lenght;
            let pxw = self.pixel_width;
            let pxh = self.pixel_height;
            let imgw = self.image_width as f32;
            let imgh = self.image_height as f32;

            let perspective = Matrix4x3::from_rows(&[
                RowVector3::new(1.0 / f, 0.0, 0.0),
                RowVector3::new(0.0, 1.0 / f, 0.0),
                RowVector3::new(0.0, 0.0, -1.0),
                RowVector3::new(0.0, 0.0, 0.0),
            ]);

            let image = Matrix3::from_rows(&[
                RowVector3::new(1.0 / pxw, 0.0, imgw / 2.0),
                RowVector3::new(0.0, -1.0 / pxh, imgh / 2.0),
                RowVector3::new(0.0, 0.0, 1.0),
            ])
            .try_inverse()
            .unwrap();

            perspective * image
        };

        let transform = extrinsics * intrinsics;
        let origin = self.translation;

        RayCaster { origin, transform }
    }
}
