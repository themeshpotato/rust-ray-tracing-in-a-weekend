use crate::math::*;
use crate::ray::*;
use crate::hittable::*;

pub enum Material {
    Lambertian { albedo: Color }    
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &Color) -> Option<Ray> {
        match self {
            Material::Lambertian { albedo } => None
        }
    }
}

pub struct Materials {
    pub materials: Vec<Material>
}

#[derive(Copy, Clone)]
pub struct MaterialHandle(pub usize); // Index into materials vec


