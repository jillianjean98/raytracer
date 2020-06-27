use crate::vectors::vec3::vec3::Vec3;
use crate::vectors::vec3::point3::Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray {
            origin, 
            direction
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.direction
    }
}