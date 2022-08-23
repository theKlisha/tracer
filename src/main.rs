use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::ops;

fn main() {
    let camera = Camera::new();
    let renderer = Renderer::new();
    renderer.render(&camera).save("./out.png").unwrap();
}

pub struct Renderer {
    // renderer settings
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, camera: &Camera) -> RgbImage {
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
                let u = (x as f64 / width as f64 * 2.0) - 1.0;
                let v = (y as f64 / height as f64 * -2.0) + 1.0;
                let ray = camera.ray(u, v);

                img.put_pixel(x, y, ray.color());
                progress_bar.inc(1);
            }
        }

        img
    }
}

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_len: f64,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            viewport_width: 16.0 / 9.0,
            viewport_height: 1.0,
            focal_len: 0.5,
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        debug_assert!(u >= -1.0 && u <= 1.0, "position outside viewport");
        debug_assert!(v >= -1.0 && v <= 1.0, "position outside viewport");

        let direction = Vec3::new(0.0, 0.0, -self.focal_len)
            + Vec3::new(u * self.viewport_width / 2.0, 0.0, 0.0)
            + Vec3::new(0.0, v * self.viewport_height / 2.0, 0.0);

        Ray::new(self.origin, direction.unit())
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        debug_assert!(
            (direction - direction.unit()).len() < 1e-10,
            "direction is not an unit vector"
        );

        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self) -> Rgb<u8> {
        let sphere_center = Vec3::new(0.0, 0.0, -2.0);
        let sphere_radius = 1.0;
        let sphere = Sphere {
            center: sphere_center,
            radius: sphere_radius,
        };

        let hit_record = sphere.hit(self, 0.0, 100.0);

        if hit_record.is_some() {
            let hit_record = hit_record.unwrap();
            let n = hit_record.normal;

            let map = |x| ((x / 2.0 + 0.5) * 255.0) as u8;
            return Rgb([map(n.0), map(n.1), map(n.2)]);
        }

        if self.direction.1 > 0.0 {
            let t = (1.0 - self.direction.1) * 0.5;
            let t = (127.0 + t * 127.0) as u8;

            return Rgb([t, t, 255]);
        };

        Rgb([127, 127, 127])
    }
}

#[derive(Debug)]
pub enum Face {
    Front,
    Back,
}

#[derive(Debug)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub face: Face,
    pub t: f64,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64) -> Self {
        let face = match point.dot(normal) > 0.0 {
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(PartialEq, Clone, Debug, Copy, Default)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3(e0, e1, e2)
    }

    pub fn len_squared(self) -> f64 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }

    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
