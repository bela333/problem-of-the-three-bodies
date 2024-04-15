use clap::Parser;
use colors::{VectorExtensions, BLUE, GREEN, RED};
use image::RgbaImage;
use layers::{circle::Circle, Layer};
use nalgebra::{vector, Vector4};

pub mod colors;
pub mod errors;
pub mod layers;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Width of the image
    #[arg(default_value_t = 128)]
    width: u32,
    /// Height of the image
    #[arg(default_value_t = 128)]
    height: u32,
    /// Anti-Aliasing scale
    #[arg(long, default_value_t = 2)]
    aa: u32,
}

fn render_to_image<L: Layer>(image: &mut RgbaImage, width: u32, height: u32, aa: u32, layer: L) {
    let aspect_ratio = width as f32 / height as f32;
    for (x, y, p) in image.enumerate_pixels_mut() {
        let x = ((x as f32 / width as f32) * 2.0 - 1.0) * aspect_ratio;
        let y = (y as f32 / height as f32) * 2.0 - 1.0;
        let mut acc: Vector4<f32> = Vector4::zeros();
        for aa_x in 0..aa {
            let x = x + 2.0 * aa_x as f32 / width as f32 / aa as f32;
            for aa_y in 0..aa {
                let y = y + 2.0 * aa_y as f32 / height as f32 / aa as f32;
                let color = layer.render(vector![x, y], 0);
                acc += color;
            }
        }
        let acc = acc / (aa as f32).powi(2);
        let acc = acc.to_srgb();
        let [r, g, b, a] = acc
            .as_ref()
            .map(|comp| (comp.min(1.0).max(0.0) * 255.0) as u8);
        p.0 = [r, g, b, a];
    }
}

fn main() -> errors::Result<()> {
    let args = Args::parse();
    let width = args.width;
    let height = args.height;
    let aa = args.aa;

    let mut img = RgbaImage::new(width, height);
    let red_circle = Circle::new(vector![0.0, 0.0], 0.1, RED);
    let green_circle = Circle::new(vector![0.1, 0.0], 0.1, GREEN.set_alpha(0.5));
    let blue_circle = Circle::new(vector![0.05, -0.1], 0.1, BLUE.set_alpha(0.5));

    let layers = red_circle.over(green_circle).over(blue_circle);

    render_to_image(&mut img, width, height, aa, layers);

    img.save("image.png")?;

    Ok(())
}
