use std::io::{self, Write};
mod vectors;

fn ray_color(/*r: vectors::ray::Ray*/) ->  () {//vectors::vec3::color::Color {
   // let unit_direction = vectors::vec3::unit_vector(r.direction());
   vectors::color::write_color(io::stdout(), vectors::vec3::vec3::Vec3::new(0.0,1.0,0.0));
}

// color ray_color(const ray& r) {
//     vec3 unit_direction = unit_vector(r.direction());
//     auto t = 0.5*(unit_direction.y() + 1.0);
//     return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
// }

fn main() {
    ray_color()
}