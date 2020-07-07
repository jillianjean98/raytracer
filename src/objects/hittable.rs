use crate::utils::ray::Ray;
use crate::utils::vec3;
use crate::objects::material::Material;
use crate::objects::material::Lambertian;
use std::option;
use std::rc::Rc;

// #[derive(Clone)]
pub struct HitRecord {
    pub p: vec3::point3::Point3,
    pub normal: vec3::vec3::Vec3,
    pub t: f64,
    pub front_face: bool, 
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(p: vec3::point3::Point3, normal: vec3::vec3::Vec3, t: f64, m: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            p, normal, t, 
            front_face: false,
            material: m
        }
    }

    pub fn new_default() -> HitRecord {
        //let &m = &Metal::new();
        HitRecord {
            p: vec3::point3::Point3::new(0.0,0.0,0.0),
            normal: vec3::vec3::Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: false,
            material: Rc::new(Lambertian::new(vec3::color::Color::new(0.0,0.0,0.0)))
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: vec3::vec3::Vec3) {
        let front_face = vec3::vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if front_face { outward_normal } else { -outward_normal};
    }

    pub fn get_t(&mut self) -> f64 {
        self.t
    }

    // pub fn get_mat(&mut self) -> Box<dyn Material> {
    //     self.material
    // }

    // pub fn new_fields(&mut self, other: &HitRecord) -> HitRecord {
    //     HitRecord {
    //         p : other.p,
    //         normal : other.normal,
    //         t : other.t,
    //         front_face : other.front_face,
    //         material : other.material
    //     }
    // }
}

pub trait Hittable {
    fn hit(&self, r : Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}