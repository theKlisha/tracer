use crate::ray::{Ray, Vec3};
use derive_builder::Builder;
use nalgebra::{Matrix3, Matrix4, Matrix4x3, RowVector3, Vector3};

pub trait Camera {
    fn ray(&self, u: f64, v: f64) -> Ray;
}

#[derive(Builder)]
pub struct HomogenousCamera {
    image_width: u32,
    image_height: u32,
    focal_lenght: f64,
    pixel_width: f64,
    pixel_height: f64,
    translation: Matrix4<f64>,
    rotation: Matrix4<f64>,

    #[builder(setter(custom))]
    extrinsic_transform: Matrix4<f64>,

    #[builder(setter(custom))]
    intrinsic_transform: Matrix4x3<f64>,

    #[builder(setter(custom))]
    camera_transform: Matrix4x3<f64>,
}

impl HomogenousCameraBuilder {
    pub fn image_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.image_width = Some(width);
        self.image_height = Some(height);

        self
    }

    pub fn extrinsic_transform(&mut self) -> &mut Self {
        let translation = self.translation.unwrap();
        let rotation = self.rotation.unwrap();

        self.extrinsic_transform =
            Some(translation.try_inverse().unwrap() * rotation.try_inverse().unwrap());

        self
    }

    pub fn intrinsic_transform(&mut self) -> &mut Self {
        let f = self.focal_lenght.unwrap();
        let pxw = self.pixel_width.unwrap();
        let pxh = self.pixel_height.unwrap();
        let imgw = self.image_width.unwrap() as f64;
        let imgh = self.image_height.unwrap() as f64;

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
    fn ray(&self, u: f64, v: f64) -> Ray {
        let r = self.camera_transform * Vector3::new(u, v, 1.0);
        let origin = Vector3::new(0.0, 0.0, self.focal_lenght);
        let direction = Vector3::from_homogeneous(r).unwrap().normalize();

        Ray::new(Vec3::from(origin), Vec3::from(direction))
    }
}
