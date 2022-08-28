use derive_builder::Builder;
use nalgebra::{Matrix3, Matrix4, Matrix4x3, RowVector3, Vector3};

use crate::Ray;

pub trait Camera {
    fn ray(&self, u: f32, v: f32) -> Ray;
}

#[derive(Builder)]
pub struct HomogenousCamera {
    image_width: u32,
    image_height: u32,
    focal_lenght: f32,
    pixel_width: f32,
    pixel_height: f32,
    translation: Matrix4<f32>,
    rotation: Matrix4<f32>,

    #[builder(setter(custom))]
    extrinsic_transform: Matrix4<f32>,

    #[builder(setter(custom))]
    intrinsic_transform: Matrix4x3<f32>,

    #[builder(setter(custom))]
    camera_transform: Matrix4x3<f32>,
}

impl HomogenousCameraBuilder {
    pub fn image_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.image_width = Some(width);
        self.image_height = Some(height);

        self
    }

    pub fn extrinsic_transform(&mut self) -> &mut Self {
        let translation_inv = Matrix4::new_translation(&self.translation.unwrap()).try_inverse().unwrap();
        let rotation_inv = self.rotation.unwrap().try_inverse().unwrap();

        self.extrinsic_transform =
            Some(translation_inv * rotation_inv);

        self
    }

    pub fn intrinsic_transform(&mut self) -> &mut Self {
        let f = self.focal_lenght.unwrap();
        let pxw = self.pixel_width.unwrap();
        let pxh = self.pixel_height.unwrap();
        let imgw = self.image_width.unwrap() as f32;
        let imgh = self.image_height.unwrap() as f32;

        let perspective = Matrix4x3::from_rows(&[
            RowVector3::new(1.0 / f, 0.0, 0.0),
            RowVector3::new(0.0, 1.0 / f, 0.0),
            RowVector3::new(0.0, 0.0, 1.0),
            RowVector3::new(0.0, 0.0, 0.0),
        ]);

        let image = Matrix3::from_rows(&[
            RowVector3::new(1.0 / pxw, 0.0, imgw / 2.0),
            RowVector3::new(0.0, -1.0 / pxh, imgh / 2.0),
            RowVector3::new(0.0, 0.0, 1.0),
        ]);

        self.intrinsic_transform = Some(perspective * image.try_inverse().unwrap());

        self
    }

    pub fn camera_transform(&mut self) -> &mut Self {
        self.camera_transform =
            Some(self.extrinsic_transform.unwrap() * self.intrinsic_transform.unwrap());

        self
    }
}

impl Camera for HomogenousCamera {
    fn ray(&self, u: f32, v: f32) -> Ray {
        let r = self.camera_transform * Vector3::new(u, v, 1.0);
        let origin = self.translation;
        let direction = Vector3::from_homogeneous(r).unwrap().normalize();

        Ray::new(origin, direction)
    }
}
