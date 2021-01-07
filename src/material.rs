use crate::math::*;
use crate::ray::*;
use crate::hittable::*;

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { albedo } => Self::lambertian_scatter(albedo, ray, rec),
            Material::Metal { albedo, fuzz } => Self::metal_scatter(albedo, *fuzz, ray, rec)
        }
    }

    fn lambertian_scatter(albedo: &Color, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        let scattered = Ray::new(rec.point, scatter_direction);

        // Catch degenerate scatter_direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        Some((scattered, *albedo))
    }
    
    fn metal_scatter(albedo: &Color, fuzz: f64, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vector3::reflect(&Vector3::normalize(&ray.direction), &rec.normal);
        let with_fuzz = reflected + fuzz * Vector3::random_in_unit_sphere();
        let scattered = Ray::new(rec.point, with_fuzz);
        
        if Vector3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, *albedo))
        } else {
            None
        }
    }
}

pub struct Materials {
    pub materials: Vec<Material>
}

#[derive(Default, Copy, Clone)]
pub struct MaterialHandle(pub usize); // Index into materials vec


