use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

use crate::traits::{Hittable, RayCaster};

pub struct DebugNormalRenderer;

impl DebugNormalRenderer {
    pub fn new() -> Self {
        DebugNormalRenderer
    }

    pub fn render(
        &self,
        ray_caster: &dyn RayCaster,
        hittable: &dyn Hittable,
        width: u32,
        height: u32,
    ) -> RgbImage {
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
                let u: f64 = f64::from(x) / f64::from(width);
                let v: f64 = f64::from(y) / f64::from(height);
                let ray = ray_caster.cast(u, v);
                let record = hittable.hit(&ray, 0.0, 100.0);

                let color = match record {
                    None if ray.direction.y > 0.0 => Rgb([0xAA, 0xAA, 0xAA]),
                    None => Rgb([0x77, 0x77, 0x77]),
                    Some(hit_record) => {
                        let mut channels = [0u8; 3];
                        channels
                            .iter_mut()
                            .zip(hit_record.normal.iter())
                            .for_each(|(c, n)| *c = ((n / 2.0 + 0.5) * 255.0) as u8);

                        Rgb(channels)
                    }
                };

                img.put_pixel(x, y, color);
                progress_bar.inc(1);
            }
        }

        img
    }
}
