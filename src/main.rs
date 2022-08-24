mod camera;
mod ray;

use camera::{Camera, HomogenousCameraBuilder};
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{Matrix4, Vector3};

fn main() {
    let camera = HomogenousCameraBuilder::default()
        .translation(Matrix4::new_translation(&Vector3::new(0.0, 0.0, 0.0)))
        .rotation(Matrix4::from_euler_angles(0.0, 180f64.to_radians(), 0.0))
        .image_size(800, 450)
        .focal_lenght(1.0)
        .pixel_width(1.0 / 450.0)
        .pixel_height(1.0 / 450.0)
        .extrinsic_transform()
        .intrinsic_transform()
        .camera_transform()
        .build()
        .unwrap();

    let renderer = Renderer::new();
    renderer.render(&camera).save("./out.png").unwrap();
}

pub struct Renderer {
    // renderer settings
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, camera: &dyn Camera) -> RgbImage {
        let width = 800;
        let height = 450;

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
                let ray = camera.ray(x as f64, y as f64);
                img.put_pixel(x, y, ray.color());

                progress_bar.inc(1);
            }
        }

        img
    }
}
