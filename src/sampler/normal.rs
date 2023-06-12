use image::Rgba;

use crate::{
    traits::{Hittable, Sampler},
    Ray,
};

pub struct NormalSampler;

impl Sampler<Rgba<u8>> for NormalSampler {
    fn sample(&self, ray: &Ray, hittable: &Box<dyn Hittable>) -> Rgba<u8> {
        match hittable.hit(ray, 0.0, 100.0) {
            None => Rgba([0, 0, 0, 0]),
            Some(hit_record) => Rgba([
                ((hit_record.normal().x / 2.0 + 0.5) * 255.0) as u8,
                ((hit_record.normal().y / 2.0 + 0.5) * 255.0) as u8,
                ((hit_record.normal().z / 2.0 + 0.5) * 255.0) as u8,
                0xFF,
            ]),
        }
    }
}
