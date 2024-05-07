use std::path::Path;

use clap::Parser;
use colors::{VectorExtensions, BLUE, GREEN, RED};
use image::RgbaImage;
use layers::{circle::Circle, glow::Glow, glowing_orb, solid::Solid, Layer};
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

fn render_to_exr<L: Layer + Sync, P: AsRef<Path>>(path: P, width: u32, height: u32, aa: u32, layer: L) -> errors::Result<()> {
    let aspect_ratio = width as f32 / height as f32;
    exr::image::write::write_rgba_file(path, width as usize, height as usize, |x, y|{
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
        (acc.x, acc.y, acc.z, acc.w)
        
    })?;
    Ok(())
}

fn main() -> errors::Result<()> {
    let args = Args::parse();
    let width = args.width;
    let height = args.height;
    let aa = args.aa;

    
    let bg = Solid::new(vector![0.0, 0.0, 0.0, 1.0]);
    
    let radius = 0.05;
    
    let red_orb = glowing_orb::new(vector![0.0, 0.0], radius, (RED*10.0).set_alpha(1.0));
    let blue_orb = glowing_orb::new(vector![0.5, 0.5], radius, (BLUE*10.0).set_alpha(1.0));
    let green_orb = glowing_orb::new(vector![-0.5, 0.5], radius, (GREEN*10.0).set_alpha(1.0));
    
    let layers = bg.over(red_orb).over(blue_orb).over(green_orb);
    
    //let mut img = RgbaImage::new(width, height);
    //render_to_image(&mut img, width, height, aa, layers);
    //img.save("image.png")?;
    render_to_exr("image.exr", width, height, aa, layers)?;


    Ok(())
}
