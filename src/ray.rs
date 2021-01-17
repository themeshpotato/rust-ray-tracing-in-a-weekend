use crate::math::*;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: f64
}

impl Ray {
    pub fn with_time(origin: Point3, direction: Vector3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time
        }
    }


    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
