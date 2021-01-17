use crate::math::*;
use crate::perlin::Perlin;

pub enum Texture {
    SolidColor(Color),
    Checker(Color, Color),
    Noise(Perlin, f64),
    Image(usize, usize, usize, Vec<u8>) // width, height, bytes per scanline, data
}

impl Texture {
    pub fn load_image(path: &str) -> Texture {
        let img = match stb_image::image::load(path) {
            stb_image::image::LoadResult::Error(err) => {
                panic!(err);
            },
            stb_image::image::LoadResult::ImageU8(image) => image,
            stb_image::image::LoadResult::ImageF32(_) => { panic!("Wrong image format!") }
        };

        Texture::Image(img.width as usize, img.height as usize, 3 * img.width as usize, img.data)
    }
}

pub trait ColorValue {
    fn get_color_value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl ColorValue for Texture {
    fn get_color_value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Texture::SolidColor(color) => {
                *color
            },
            Texture::Checker(even, odd) => {
                let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
                if sines < 0.0 {
                    *odd
                } else {
                    *even
                }
            },
            Texture::Noise(perlin, scale) => {
                Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (scale * p.z + 10.0 * perlin.turb(p, 7)).sin())
            },
            Texture::Image(w, h, bytes_per_scanline, data) => {
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = clamp(u, 0.0, 1.0);
                let v = 1.0 - clamp(v, 0.0, 1.0); // Flip V to image coordinates

                //eprintln!("U {} V {}", u, v);
                
                let mut i = (u * *w as f64) as usize;
                let mut j = (v * *h as f64) as usize;

                // Clamp integer mapping, since actual coordinates should be less than 1.0
                if i >= *w {
                    i = w - 1;
                }

                if j >= *h {
                    j = h - 1;
                }

                let color_scale = 1.0 / 255.0;
                let pixel: [f64; 3] = unsafe {
                    let ptr: *const u8 = data.as_ptr().offset((j * bytes_per_scanline + i * 3) as isize);

                    [color_scale * *ptr as f64, color_scale * *ptr.offset(1) as f64, color_scale * *ptr.offset(2) as f64]
                };

                Color::new(pixel[0], pixel[1], pixel[2])
            }
        }
    }
}
