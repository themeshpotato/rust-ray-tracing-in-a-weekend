use crate::math::*;
use crate::ray::*;
use crate::hittable::*;
use crate::texture::*;

pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ir: f64 }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { albedo } => Self::lambertian_scatter(albedo, ray, rec),
            Material::Metal { albedo, fuzz } => Self::metal_scatter(albedo, *fuzz, ray, rec),
            Material::Dielectric { ir } => Self::dielectric_scatter(*ir, ray, rec)
        }
    }

    fn lambertian_scatter(albedo: &Texture, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        // Catch degenerate scatter_direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::with_time(rec.point, scatter_direction, ray.time);


        let attenuation = albedo.get_color_value(rec.u, rec.v, &rec.point);
        
        Some((scattered, attenuation))
    }
    
    fn metal_scatter(albedo: &Color, fuzz: f64, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vector3::reflect(&Vector3::normalize(&ray.direction), &rec.normal);
        let with_fuzz = reflected + fuzz * Vector3::random_in_unit_sphere();
        let scattered = Ray::with_time(rec.point, with_fuzz, ray.time);
        
        if Vector3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, *albedo))
        } else {
            None
        }
    }

    fn dielectric_scatter(ir: f64, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / ir } else { ir };

        let unit_direction = Vector3::normalize(&ray.direction);
        let cos_theta = Vector3::dot(&(-unit_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = {
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                Vector3::reflect(&unit_direction, &rec.normal)
            } else {
                Vector3::refract(&unit_direction, &rec.normal, refraction_ratio)
            }
        };
        
        let scattered = Ray::with_time(rec.point, direction, ray.time);

        Some((scattered, attenuation))
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

#[derive(Default, Copy, Clone)]
pub struct MaterialHandle(pub usize); // Index into materials vec


