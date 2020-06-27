use std::io::{self, Write};
mod vectors;
use vectors::vec3::color::Color as Color;
use vectors::vec3::point3::Point3 as Point3;
use vectors::vec3::vec3::Vec3 as Vec3;

fn hit_sphere(center: Point3, radius: f64, ray: vectors::ray::Ray) -> f64 {
    // 𝑡2𝐛⋅𝐛+2𝑡𝐛⋅(𝐀−𝐂)+(𝐀−𝐂)⋅(𝐀−𝐂)−𝑟2=0
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = vectors::vec3::vec3::dot(oc, ray.direction);
    let c = oc.length_squared() - radius*radius;
    let discrim = half_b*half_b - a*c;
    if discrim < 0.0 {
        -1.0
    } else {
        (-half_b - discrim.sqrt()) / a
    }
}

fn ray_color(r: vectors::ray::Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let n = vectors::vec3::vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
        0.5*Color::new(1.0, 1.0, n.z() + 1.0)
    } else {
        let unit_direction = vectors::vec3::vec3::unit_vector(r.direction);
        t = 0.5*(unit_direction.y() + 1.0);
        (1.0-t)*Color::new(1.0,1.0,1.0) + t*Color::new(0.7, 0.3, 1.0)
    }  
}

// color ray_color(const ray& r) {
//     vec3 unit_direction = unit_vector(r.direction());
//     auto t = 0.5*(unit_direction.y() + 1.0);
//     return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
// }

fn main() -> io::Result<()> {
    const ASPECT_RATIO : f64= 16.0 / 9.0;
    const IMAGE_WIDTH : i32 = 384;
    const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as i32;
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    
    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
    // auto horizontal = vec3(viewport_width, 0, 0);
    // auto vertical = vec3(0, viewport_height, 0);
    // auto lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush()?;
        let mut i = 0;
        while i < IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = vectors::ray::Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            vectors::color::write_color(io::stdout(), pixel_color)?;
            i += 1;
        }
        j -=1;
    }
    eprintln!("\nDone.");
    Ok(())
    // for (int j = image_height-1; j >= 0; --j) {
    //     std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
    //     for (int i = 0; i < image_width; ++i) {
    //         auto u = double(i) / (image_width-1);
    //         auto v = double(j) / (image_height-1);
    //         ray r(origin, lower_left_corner + u*horizontal + v*vertical - origin);
    //         color pixel_color = ray_color(r);
    //         write_color(std::cout, pixel_color);
    //     }
    // }

    // std::cerr << "\nDone.\n";
}

