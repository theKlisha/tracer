use crate::{renderer::HitRecord, Ray};

pub trait RayCaster {
    fn cast(&self, u: f64, v: f64) -> Ray;
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait Sampler<T> {
    fn sample(&self, ray: &Ray, hittable: &Box<dyn Hittable>) -> T;
}
