use std::ops::Mul;

use crate::{
    math::{Color, Point2, Vector2},
    renderer::material::{LambertianMaterial, Material},
    traits::{Hittable, RayCaster, Sampler},
    Ray,
};

pub struct SimpleSampler {
    pub max_depth: u32,
    pub ambient_light: Color,
    pub subsamples: u32,

    // TODO: move material to the hittable
    pub material: Box<dyn Material>,
}

impl SimpleSampler {
    fn inner_sample(&self, hittable: &Box<dyn Hittable>, ray: &Ray, depth: u32) -> Color {
        if let Some(interaction) = hittable.hit(ray, 0.0, 100.0) {
            let (color, scattered_ray) = self.material.scatter(&interaction);

            if let Some(scattered_ray) = scattered_ray {
                if depth > 0 {
                    color * self.inner_sample(hittable, &scattered_ray, depth - 1)
                } else {
                    Color::new(0.0, 0.0, 0.0)
                }
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            return self.ambient_light.clone();
        }
    }
}

impl Default for SimpleSampler {
    fn default() -> Self {
        Self {
            max_depth: 3,
            ambient_light: Color::new(1.0, 1.0, 1.0),
            subsamples: 10,

            material: Box::new(LambertianMaterial {
                albedo: Color::new(0.6, 0.8, 0.9),
            }),
        }
    }
}

impl Sampler for SimpleSampler {
    fn sample(
        &self,
        hittable: &Box<dyn Hittable>,
        ray_caster: &Box<dyn RayCaster>,
        p: &Point2,
        uv: &Vector2,
    ) -> Color {
        let ray = ray_caster.cast(p + uv);

        (0..self.subsamples)
            .map(|_| self.inner_sample(hittable, &ray, self.max_depth))
            .sum::<Color>()
            .mul(1.0 / self.subsamples as f64)
    }
}
