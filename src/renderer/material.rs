use crate::{algorythms::random_unit_vector, math::Color, Ray};

use super::SurfaceInteraction;

pub struct DebugNormalMaterial;

impl Material for DebugNormalMaterial {
    fn scatter(&self, interaction: &SurfaceInteraction) -> (Color, Option<Ray>) {
        let ray = Ray::new(interaction.point, interaction.normal);
        let color = Color::new(
            interaction.normal.x * 0.5 + 0.5,
            interaction.normal.y * 0.5 + 0.5,
            interaction.normal.z * 0.5 + 0.5,
        );

        (color, Some(ray))
    }
}

pub struct LambertianMaterial {
    pub albedo: Color,
}

impl Material for LambertianMaterial {
    fn scatter(&self, interaction: &SurfaceInteraction) -> (Color, Option<Ray>) {
        let scatter_direction = interaction.normal + random_unit_vector();
        let scattered_ray = Ray::new(interaction.point, scatter_direction.normalize());

        (self.albedo.clone(), Some(scattered_ray))
    }
}

pub trait Material {
    fn scatter(&self, interaction: &SurfaceInteraction) -> (Color, Option<Ray>);
}
