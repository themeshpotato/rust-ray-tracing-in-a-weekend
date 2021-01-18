use crate::math::*;
use crate::ray::*;
use crate::hittable::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> AABB {
        AABB {
            minimum: a,
            maximum: b
        }
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z)
        );

        let big = Point3::new(
            f64::max(box0.maximum.x, box1.maximum.x),
            f64::max(box0.maximum.y, box1.maximum.y),
            f64::max(box0.maximum.z, box1.maximum.z)
        );

        AABB::new(small, big)
    }

    fn box_compare(a: &Hittable, b: &Hittable, axis: i32) -> std::cmp::Ordering {
        if let (Some(box_a), Some(box_b)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
            let a_min = box_a.minimum.as_array();
            let b_min = box_b.minimum.as_array();

            if a_min[axis as usize] < b_min[axis as usize] { 
                std::cmp::Ordering::Less 
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Equal
        }
    }

    pub fn box_x_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    pub fn box_y_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    pub fn box_z_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }

    #[allow(dead_code)]
    fn min_max(a: f64, b: f64, min: &mut f64, max: &mut f64) -> bool {
        let t0 = a.min(b); 
        let t1 = a.max(b);
        *min = t0.max(*min);
        *max = t1.min(*max);

        if max <= min {
            return true;
        }

        false
    }
    
    #[allow(dead_code)]
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut min = t_min;
        let mut max = t_max;
        let minimum = self.minimum.as_array();
        let maximum = self.maximum.as_array();
        let ray_origin = ray.origin.as_array();
        let ray_direction = ray.direction.as_array();

        for a in 0..3 {
            let inv_d = 1.0 / ray_direction[a];
            let mut t0 = (minimum[a] - ray_origin[a]) * inv_d;
            let mut t1 = (maximum[a] - ray_origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            min = if t0 > min { t0 } else { min };
            max = if t1 < max { t1 } else { max };

            if max <= min {
                return false;
            }
        }

        true
    }
}
