use crate::{
    math::{Color, Point2, Vector2},
    renderer::SurfaceInteraction,
    Ray,
};

pub trait RayCaster {
    fn cast(&self, uv: Point2) -> Ray;
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<SurfaceInteraction>;
}

pub trait Sampler {
    fn sample(
        &self,
        hittable: &Box<dyn Hittable>,
        ray_caster: &Box<dyn RayCaster>,
        p: &Point2,
        uv: &Vector2,
    ) -> Color;
}
