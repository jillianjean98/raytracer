use crate::utils::ray::Ray;
use crate::utils::vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: vec3::point3::Point3,
    pub normal: vec3::vec3::Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: vec3::point3::Point3, normal: vec3::vec3::Vec3, t: f64) -> HitRecord {
        HitRecord {
            p, normal, t, 
            front_face: false
        }
    }

    pub fn new_default() -> HitRecord {
        HitRecord {
            p: vec3::point3::Point3::new(0.0,0.0,0.0),
            normal: vec3::vec3::Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: vec3::vec3::Vec3) {
        let front_face = vec3::vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if front_face { outward_normal } else { -outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r : Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}