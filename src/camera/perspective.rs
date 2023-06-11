use crate::{
    math::{Matrix4, Matrix4x3, Point3, Vector3, RowVector3, Matrix3},
    traits::RayCaster,
    Ray,
};
use derive_builder::Builder;

#[derive(Builder)]
pub struct PerspectiveCamera {
    #[builder(default = "1.0")]
    aspect_ratio: f64,

    #[builder(default = "1.0")]
    focal_lenght: f64,

    translation: Vector3,
    rotation_matrix: Matrix4,
}

pub struct PerspectiveCameraCaster {
    origin: Point3,
    transform: Matrix4x3,
}

impl RayCaster for PerspectiveCameraCaster {
    fn cast(&self, u: f64, v: f64) -> Ray {
        let direction = self.transform * Vector3::new(u, v, 1.0);
        let direction = nalgebra::Vector3::from_homogeneous(direction)
            .unwrap()
            .normalize();
        Ray::new(self.origin, direction)
    }
}

impl PerspectiveCameraBuilder {
    pub fn aspect_frac<T>(&mut self, width: T, height: T) -> &mut Self
    where
        f64: From<T>,
    {
        let ratio = f64::from(width) / f64::from(height);
        self.aspect_ratio(ratio);
        self
    }

    pub fn focal_lenght_mm(&mut self, focal_lenght: u32) -> &mut Self {
        let focal_lenght = focal_lenght as f64 / 1000.0;
        self.focal_lenght(focal_lenght);
        self
    }

    pub fn look_at(&mut self, target: Point3) -> &mut Self {
        let matrix = Matrix4::look_at_rh(
            &self.translation.expect("no translation provided").into(),
            &target,
            &Vector3::new(0.0, 1.0, 0.0),
        );

        self.rotation_matrix(matrix);
        self
    }
}

impl PerspectiveCamera {
    pub fn ray_caster(&self) -> PerspectiveCameraCaster {
        let extrinsics = {
            let translation_inv = Matrix4::new_translation(&self.translation)
                .try_inverse()
                .unwrap();
            let rotation_inv = self.rotation_matrix.try_inverse().unwrap();
            translation_inv * rotation_inv
        };

        let intrinsics = {
            let f = self.focal_lenght;
            let pxw = self.aspect_ratio;
            let pxh = 1.0;
            let imgw = 1.0;
            let imgh = 1.0;

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

        PerspectiveCameraCaster {
            origin: origin.into(),
            transform,
        }
    }
}
