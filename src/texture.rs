use crate::math::*;

pub enum Texture {
    SolidColor(Color),
    Checker(Color, Color)
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
            }
        }
    }
}
