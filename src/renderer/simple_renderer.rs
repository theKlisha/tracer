use image::RgbaImage;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    math::{Point2, Vector2},
    traits::{Hittable, RayCaster, Sampler},
};

pub fn render(
    ray_caster: &Box<dyn RayCaster>,
    hittable: &Box<dyn Hittable>,
    sampler: &Box<dyn Sampler>,
    width: u32,
    height: u32,
) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);
    let progress_bar = ProgressBar::new((height * height) as u64);

    progress_bar.set_style(
        ProgressStyle::with_template(
            "{wide_bar} {percent}% [{elapsed_precise}] / [{duration_precise}]",
        )
        .unwrap(),
    );

    for y in 0..height {
        for x in 0..width {
            let p = Point2::new(x as f64 / width as f64, y as f64 / height as f64);
            let uv = Vector2::new(0.0, 0.0);
            let color = sampler.sample(&hittable, &ray_caster, &p, &uv);
            img.put_pixel(x, y, color.clip_exposure(2.0));
            progress_bar.inc(1);
        }
    }

    img
}
