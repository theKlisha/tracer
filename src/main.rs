mod camera;

use camera::{Camera, HomogenousCameraBuilder};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{Matrix4, Vector3};

fn main() {
    let sphere = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let camera = HomogenousCameraBuilder::default()
        .translation(Matrix4::new_translation(&Vector3::new(0.0, 0.0, 0.0)))
        .rotation(Matrix4::from_euler_angles(0.0, 180f32.to_radians(), 0.0))
        .image_size(800, 450)
        .focal_lenght(1.0)
        .pixel_width(1.0 / 450.0)
        .pixel_height(1.0 / 450.0)
        .extrinsic_transform()
        .intrinsic_transform()
        .camera_transform()
        .build()
        .unwrap();

    let renderer = Renderer::new();
    renderer.render(&camera, &sphere).save("./out.png").unwrap();
}

pub struct Renderer {
    // renderer settings
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, camera: &dyn Camera, hittable: &dyn Hittable) -> RgbImage {
        let width = 800;
        let height = 450;

        let mut img = RgbImage::new(width, height);
        let progress_bar = ProgressBar::new((height * height) as u64);
        progress_bar.set_style(
            ProgressStyle::with_template(
                "{wide_bar} {percent}% [{elapsed_precise}] / [{duration_precise}]",
            )
            .unwrap(),
        );

        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray(x as f32, y as f32);
                let record = hittable.hit(&ray, 0.0, 100.0);
                let color = match record {
                    Some(hit_record) => {
                        let mut channels = [0u8; 3];
                        channels
                            .iter_mut()
                            .zip(hit_record.normal.iter())
                            .for_each(|(c, n)| *c = ((n / 2.0 + 0.5) * 255.0) as u8);

                        Rgb(channels)
                    }
                    None => Rgb([0, 0, 0]),
                };

                img.put_pixel(x, y, color);
                progress_bar.inc(1);
            }
        }

        img
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        debug_assert!(
            (direction - direction.normalize()).magnitude() < 1e-5,
            "direction is not an unit vector"
        );

        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + (self.direction * t)
    }
}

#[derive(Debug)]
pub enum Face {
    Front,
    Back,
}

#[derive(Debug)]
pub struct HitRecord {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub face: Face,
    pub t: f32,
}

impl HitRecord {
    pub fn new(point: Vector3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        let face = match point.dot(&normal) > 0.0 {
            true => Face::Front,
            false => Face::Back,
        };

        Self {
            point,
            normal,
            face,
            t,
        }
    }
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let t = -half_b - discriminant.sqrt() / a;

        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(point, normal, t))
    }
}
