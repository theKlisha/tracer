use image::{Rgba, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};

use crate::traits::{Hittable, RayCaster, Sampler};

pub fn render<R>(
    ray_caster: &R,
    hittable: &Box<dyn Hittable>,
    sampler: &Box<dyn Sampler<Rgba<u8>>>,
    width: u32,
    height: u32,
) -> RgbaImage
where
    R: RayCaster,
{
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
            let u: f64 = f64::from(x) / f64::from(width);
            let v: f64 = f64::from(y) / f64::from(height);
            let ray = ray_caster.cast(u, v);
            let color = sampler.sample(&ray, &hittable);

            img.put_pixel(x, y, color);
            progress_bar.inc(1);
        }
    }

    img
}
