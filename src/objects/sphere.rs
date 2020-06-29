
pub mod sphere {
    use crate::objects::hittable;
    use crate::objects::hittable::Hittable;
    use crate::utils;
    use crate::utils::ray::Ray;

    pub struct Sphere {
        center: utils::vec3::point3::Point3,
        radius: f64
    }

    impl Sphere {
        pub fn new(center: utils::vec3::point3::Point3, radius: f64) -> Sphere {
            Sphere {
                center,
                radius
            }
        }
    }

    impl Hittable for Sphere {
        fn hit(&self, r : Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
            let oc = r.origin - self.center;
            let a = r.direction.length_squared();
            let half_b = utils::vec3::vec3::dot(oc, r.direction);
            let c = oc.length_squared() - self.radius*self.radius;
            let discrim = half_b*half_b - a*c;
            if discrim > 0.0 {
                let root = discrim.sqrt();
                let mut temp = (-half_b - root)/a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.at(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    rec.set_face_normal(r, rec.normal);
                    true
                } else {
                    temp = (-half_b + root)/a;
                    if temp < t_max && temp > t_min {
                        rec.t = temp;
                        rec.p = r.at(rec.t);
                        rec.normal = (rec.p - self.center) / self.radius;
                        rec.set_face_normal(r, rec.normal);
                        true
                    } else {
                        false
                    }
                }
            } else {
                //eprint!("case4 {}", discrim);
                false
            }
        }
    }
}