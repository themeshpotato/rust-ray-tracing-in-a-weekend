use crate::math::*;
use crate::ray::*;
use crate::material::*;
use crate::aabb::*;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub mat_handle: MaterialHandle
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            ..Default::default()
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = Vector3::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -outward_normal };
    }
}

pub enum Hittable {
    Sphere { mat_handle: MaterialHandle, center: Point3, radius: f64 },
    MovingSphere { mat_handle: MaterialHandle, center_0: Point3, center_1: Point3, time_0: f64, time_1: f64, radius: f64 },
    BvhNode { left_index: usize, right_index: usize, aabb_box: AABB }
}

pub fn hit_hittables(hittables: &Vec<Hittable>, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut rec: Option<HitRecord> = None;

    for hittable in hittables {
        if let Some(record) = hittable.hit(ray, t_min, closest_so_far) {
            closest_so_far = record.t;
            rec = Some(record)
        }
    }
    
    rec
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere { mat_handle, center, radius } => {
                Self::sphere_hit(&center, *radius, ray, t_min, t_max, *mat_handle)
            },
            Hittable::MovingSphere { mat_handle, center_0, center_1, time_0, time_1, radius } => {
                Self::sphere_hit(&Self::get_center_at_time(center_0, center_1, *time_0, *time_1, ray.time), *radius, ray, t_min, t_max, *mat_handle)
            },
            Hittable::BvhNode { left_index, right_index, aabb_box } => {
                None
            }
        }
    }

    fn sphere_hit(center: &Point3, radius: f64, ray: &Ray, t_min: f64, t_max: f64, mat_handle: MaterialHandle) -> Option<HitRecord> {
        let oc = ray.origin - *center;
        let a = ray.direction.length_squared();
        let half_b = Vector3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
             return None;
        } 

        let sqrtd = discriminant.sqrt();
        
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let mut rec = HitRecord::new();

        rec.mat_handle = mat_handle;
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - *center) / radius;
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }

    fn sphere_bounding_box(center: &Point3, radius: f64) -> Option<AABB> {
        Some(
            AABB::new(
                *center - Vector3::new(radius, radius, radius),
                *center + Vector3::new(radius, radius, radius)
            )
        )
    }

    fn moving_sphere_bounding_box(center_0: &Point3, center_1: &Point3, radius: f64, time_0: f64, time_1: f64) -> Option<AABB> {
        let c0 = Self::get_center_at_time(center_0, center_1, time_0, time_1, time_0);
        let c1 = Self::get_center_at_time(center_0, center_1, time_0, time_1, time_1);

        let box0 = AABB::new(
            c0 - Vector3::new(radius, radius, radius),
            c0 + Vector3::new(radius, radius, radius)
        );

        let box1 = AABB::new(
            c1 - Vector3::new(radius, radius, radius),
            c1 + Vector3::new(radius, radius, radius)
        );

        Some(AABB::surrounding_box(&box0, &box1))
    }

    fn get_center_at_time(center_0: &Point3, center_1: &Point3, time_0: f64, time_1: f64, time: f64) -> Point3 {
        *center_0 + ((time - time_0) / (time_1 - time_0)) * (*center_1 - *center_0)
    }
}
