#[path = "vec3.rs"] mod vec3;

type ray Point3 = vec3::Point3;

pub struct Ray {
    origin: Point3,
    direction: vec3::Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray {
            origin, 
            direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.direction
    }
}