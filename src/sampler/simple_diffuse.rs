use image::Rgba;
use rand::Rng;
use std::ops::Mul;

use crate::{
    math::Vector3,
    traits::{Hittable, Sampler},
    Ray,
};

pub struct SimpleDiffuseSampler {
    subsamples: u32,
    max_bounces: u32,
    far_plane: f64,
    near_plane: f64,
    ambient_light: f64,
    attenuation: f64,
    scatter: f64,
    gamma: f64,
}

impl Default for SimpleDiffuseSampler {
    fn default() -> Self {
        Self {
            subsamples: 20,
            max_bounces: 3,
            far_plane: 100.0,
            ambient_light: 0.5,
            near_plane: 0.001,
            attenuation: 0.5,
            scatter: 1.0,
            gamma: 2.0,
        }
    }
}

impl SimpleDiffuseSampler {
    fn inner_sample(&self, ray: &Ray, hittable: &Box<dyn Hittable>, depth: u32) -> f64 {
        match hittable.hit(ray, self.near_plane, self.far_plane) {
            None => self.ambient_light,
            Some(hit_record) => {
                let target = hit_record.point()
                    + hit_record.normal()
                    + random_in_unit_sphere() * self.scatter;
                let scattered = Ray::new(hit_record.point(), target - hit_record.point());

                if depth >= self.max_bounces {
                    0.0
                } else {
                    self.attenuation * self.inner_sample(&scattered, hittable, depth + 1)
                }
            }
        }
    }
}

impl Sampler<Rgba<u8>> for SimpleDiffuseSampler {
    fn sample(&self, ray: &Ray, hittable: &Box<dyn Hittable>) -> Rgba<u8> {
        let x = (0..self.subsamples)
            .map(|_| self.inner_sample(ray, hittable, 0))
            .sum::<f64>()
            .mul(self.gamma / self.subsamples as f64)
            .sqrt()
            .min(1.0)
            .max(0.0)
            .mul(255.0) as u8;

        Rgba([x, x, x, 0xFF])
    }
}

fn random_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if p.magnitude_squared() < 1.0 {
            return p.normalize();
        }
    }
}
