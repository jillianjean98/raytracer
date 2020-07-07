use crate::utils::ray::Ray;
use crate::utils::vec3;
use crate::objects::hittable;
use crate::utils::vec3::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &hittable::HitRecord, attenuation: &mut vec3::color::Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}
impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian {
            albedo: color
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &hittable::HitRecord, attenuation: &mut vec3::color::Color, scattered: &mut Ray) -> bool {
        let scatter_direction = hit_record.normal + vec3::vec3::random_unit_vector();
        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal {
            albedo: color
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &hittable::HitRecord, attenuation: &mut vec3::color::Color, scattered: &mut Ray) -> bool {
        // vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        // scattered = ray(rec.p, reflected);
        // attenuation = albedo;
        // return (dot(scattered.direction(), rec.normal) > 0);
        let reflected = vec3::vec3::reflect(vec3::vec3::unit_vector(r_in.direction), hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected);
        *attenuation = self.albedo;
        vec3::vec3::dot(scattered.direction, hit_record.normal) > 0.0
    }
}