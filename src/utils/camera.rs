pub mod camera {
    use crate::utils::vec3;
    use crate::utils::ray;

    pub struct Camera {
        origin: vec3::point3::Point3,
        lower_left_corner: vec3::point3::Point3,
        horizontal: vec3::vec3::Vec3,
        vertical: vec3::vec3::Vec3
    }

    impl Camera {
        pub fn new() -> Camera {
            let aspect_ratio = 16.0 / 9.0;
            let viewport_height = 2.0;
            let viewport_width = aspect_ratio * viewport_height;
            let focal_length = 1.0;

            let origin = vec3::point3::Point3::new(0.0, 0.0, 0.0);
            let horizontal = vec3::vec3::Vec3::new(viewport_width, 0.0, 0.0);
            let vertical = vec3::vec3::Vec3::new(0.0, viewport_height, 0.0);
            Camera {
                origin,
                horizontal,
                vertical,
                lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - vec3::vec3::Vec3::new(0.0, 0.0, focal_length)
            }
        }

        pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
            ray::Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
        }
    }
}