mod algorythms;
mod camera;

use algorythms::{moller_trumbore_intersection, sphere_intersection};
use camera::{Camera, PerspectiveCameraBuilder};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{Point3, Vector3};
use tobj;

fn main() {
    let scene = Scene::load_obj("./scene.obj");

    let camera = PerspectiveCameraBuilder::default()
        .image_size(800, 450)
        .focal_lenght(2.0)
        .pixel_width(1.0 / 450.0)
        .pixel_height(1.0 / 450.0)
        .translation(Vector3::new(5.0, 5.0, 10.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .build()
        .unwrap();

    let renderer = Renderer::new();
    renderer.render(&camera, &scene).save("./out.png").unwrap();
}

pub struct Renderer {
    // renderer settings
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, camera: &dyn Camera, hittable: &dyn Hittable) -> RgbImage {
        let ray_caster = camera.ray_caster();

        // TODO: use camera serrings instead of hardcoding values in renderer
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
                let ray = ray_caster.ray(x as f32, y as f32);
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
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        debug_assert!(
            (direction - direction.normalize()).magnitude() < 1e-5,
            "direction is not an unit vector"
        );

        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
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
    point: Point3<f32>,
    normal: Vector3<f32>,
    // face: Face,
    t: f32,
}

impl HitRecord {
    pub fn new(point: Point3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        // let face = match point.dot(&normal) > 0.0 {
        //     true => Face::Front,
        //     false => Face::Back,
        // };

        Self {
            point,
            normal,
            // face,
            t,
        }
    }
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = sphere_intersection(&ray.origin, &ray.direction, &self.center, self.radius);

        match t {
            Some((t0, _)) if t0 < t_min || t0 > t_max => {
                let point = ray.at(t0);
                let normal = (point - self.center) / self.radius;
                Some(HitRecord::new(point, normal, t0))
            }
            _ => None,
        }
    }
}

pub struct Triangle {
    vertices: [Point3<f32>; 3],
    normal: Vector3<f32>,
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = moller_trumbore_intersection(
            &ray.origin,
            &ray.direction,
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
        );

        t.map(|t| HitRecord::new(ray.at(t), self.normal, t))
    }
}

pub struct Mesh {
    pub faces: Vec<Triangle>,
}

impl From<&tobj::Mesh> for Mesh {
    fn from(m: &tobj::Mesh) -> Self {
        let faces = m
            .indices
            .chunks(3)
            .zip(m.normals.chunks(3))
            .map(|(indices, normal)| {
                let position = |i| {
                    let p = m.positions.chunks(3).nth(i).unwrap();
                    Point3::new(p[0], p[1], p[2])
                };

                Triangle {
                    vertices: [
                        position(indices[0] as usize),
                        position(indices[1] as usize),
                        position(indices[2] as usize),
                    ],
                    normal: Vector3::new(normal[0], normal[1], normal[2]),
                }
            })
            .collect();

        Mesh { faces }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces
            .iter()
            .map(|f| f.hit(ray, t_min, t_max))
            .fold(None, |rec, min| match (rec, min) {
                (Some(rec), Some(min)) => {
                    if rec.t < min.t {
                        Some(rec)
                    } else {
                        Some(min)
                    }
                }
                (Some(rec), None) => Some(rec),
                (None, min) => min,
            })
    }
}

pub struct Scene {
    meshes: Vec<Mesh>,
}

impl Scene {
    pub fn load_obj(file_name: &str) -> Scene {
        let obj = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS);
        let (models, _materials) = obj.unwrap();
        let meshes = models.into_iter().map(|m| Mesh::from(&m.mesh)).collect();

        Scene { meshes }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.meshes
            .iter()
            .map(|mesh| mesh.hit(ray, t_min, t_max))
            .fold(None, |rec, min| match (rec, min) {
                (Some(rec), Some(min)) => {
                    if rec.t < min.t {
                        Some(rec)
                    } else {
                        Some(min)
                    }
                }
                (Some(rec), None) => Some(rec),
                (None, min) => min,
            })
    }
}
