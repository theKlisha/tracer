pub mod algorythms;
pub mod camera;
pub mod math;
pub mod renderer;
pub mod sampler;
pub mod traits;

use algorythms::sphere_intersection;
use camera::perspective::PerspectiveCameraBuilder;
use math::{Point3, Vector3};
use renderer::SurfaceInteraction;
use sampler::simple::SimpleSampler;
use traits::{Hittable, RayCaster, Sampler};

fn main() {
    let mut hittables = Box::new(Vec::new());

    hittables.push(Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    });

    hittables.push(Sphere {
        center: Point3::new(0.0, -101.0, 0.0),
        radius: 100.0,
    });

    let camera = PerspectiveCameraBuilder::default()
        .aspect_frac(16, 9)
        .focal_lenght(2.0)
        .translation(Vector3::new(0.0, 2.0, 10.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .build()
        .expect("Failed to build camera");

    let hittables: Box<dyn Hittable> = hittables;
    let sampler: Box<dyn Sampler> = Box::new(SimpleSampler::default());
    let caster: Box<dyn RayCaster> = Box::new(camera.ray_caster());

    renderer::simple_renderer::render(&caster, &hittables, &sampler, 1920, 1080)
        .save("./out.png")
        .expect("Failed to save image");
}

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        debug_assert!(
            (direction - direction.normalize()).magnitude() < 1e-5,
            "direction is not an unit vector"
        );

        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<SurfaceInteraction> {
        let t = sphere_intersection(&ray.origin, &ray.direction, &self.center, self.radius);

        match t {
            Some((t0, _)) if t0 > t_min && t0 < t_max => {
                let point = ray.at(t0);
                let normal = (point - self.center) / self.radius;
                Some(SurfaceInteraction::new(point, ray.direction, normal, t0))
            }
            _ => None,
        }
    }
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<SurfaceInteraction> {
        self.iter()
            .filter_map(|h| h.hit(ray, t_min, t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
