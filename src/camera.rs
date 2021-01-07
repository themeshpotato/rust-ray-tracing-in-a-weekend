use crate::math::*;
use crate::ray::*;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
    pub lense_radius: f64
}

impl Camera {
    pub fn new(
            look_from: &Point3,
            look_at: &Point3,
            vup: &Vector3,
            vfov: f64,
            aspect_ratio: f64,
            aperture: f64,
            focus_dist: f64
            ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vector3::normalize(&(*look_from - *look_at));
        let u = Vector3::normalize(&Vector3::cross(&vup, &w));
        let v = Vector3::cross(&w, &u);

        let origin = *look_from; 
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v; 
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - focus_dist * w; 
        let lense_radius = aperture * 0.5;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lense_radius
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lense_radius * Vector3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset
            )
    }
}
