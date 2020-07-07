use std::io::{self, Write};
use std::f64::INFINITY;
mod utils;
mod objects;
use utils::vec3::color::Color;
use utils::vec3::point3::Point3;
use utils::vec3::vec3::Vec3;
use utils::ray::Ray;
use objects::sphere::sphere::Sphere;
use objects::hittable_list::hittable_list::HittableList;
use objects::hittable::Hittable;
use std::rc::Rc;
use crate::objects::material;

extern crate rand;
use rand::Rng;
// fn hit_sphere(center: Point3, radius: f64, ray: vectors::ray::Ray) -> f64 {
//     // 𝑡2𝐛⋅𝐛+2𝑡𝐛⋅(𝐀−𝐂)+(𝐀−𝐂)⋅(𝐀−𝐂)−𝑟2=0
//     let oc = ray.origin - center;
//     let a = ray.direction.length_squared();
//     let half_b = vectors::vec3::vec3::dot(oc, ray.direction);
//     let c = oc.length_squared() - radius*radius;
//     let discrim = half_b*half_b - a*c;
//     if discrim < 0.0 {
//         -1.0
//     } else {
//         //print!("{}", discrim);
//         (-half_b - discrim.sqrt()) / a
//     }
// }
// fn random_double() -> f64 {
//     // Returns a random real in [0,1).
//     rng.gen::<f64>()
// }

// fn random_double_rng(min: f64, max: f64) -> f64 {
//     // Returns a random real in [min,max).
//     rng.gen_range(min, max)
// }

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        Color::new(0.0,0.0,0.0)
    } else {
        //let sph = Sphere::new(Point3::new(0.0,0.5,-1.0), 0.3, Rc::new(material::Lambertian::new(Color::new(0.0,0.0,0.0))));
        //let mut t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
        let mut hr = objects::hittable::HitRecord::new_default();
        // if (world.hit(r, 0.001, infinity, rec)) {
        //     ray scattered;
        //     color attenuation;
        //     if (rec.mat_ptr->scatter(r, rec, attenuation, scattered))
        //         return attenuation * ray_color(scattered, world, depth-1);
        //     return color(0,0,0);
        // }
        if world.hit(r, 0.0001, INFINITY, &mut hr) {
            let mut scattered = Ray::new_default();
            let mut attenuation = Color::new(0.0,0.0,0.0);
            if hr.material.scatter(&r, &hr, &mut attenuation, &mut scattered) {
                attenuation * ray_color(scattered, world, depth - 1)
            } else {
                Color::new(0.0,0.0,0.0)
            }
            // let target = hr.p + hr.normal + utils::vec3::vec3::random_unit_vector();
            // //let n = utils::vec3::vec3::unit_vector(hr.p - Vec3::new(0.0,0.0,-1.0));
            // 0.5*ray_color(Ray::new(hr.p, target-hr.p), world, depth - 1)
        } else {
            let unit_direction = utils::vec3::vec3::unit_vector(r.direction);
            let t = 0.5*(unit_direction.y() + 1.0);
            (1.0-t)*Color::new(1.0,1.0,1.0) + t*Color::new(0.5, 0.7, 1.0)
        }
    }
    
    // if t > 0.0 {
    //     let n = vectors::vec3::vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
    //     0.5*Color::new(1.0, 1.0, n.z() + 1.0)
    // } else {
    //     let unit_direction = vectors::vec3::vec3::unit_vector(r.direction);
    //     t = 0.5*(unit_direction.y() + 1.0);
    //     (1.0-t)*Color::new(1.0,1.0,1.0) + t*Color::new(0.7, 0.3, 1.0)
    // }  
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
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    // let origin = Point3::new(0.0,0.0,0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    //let vertical = Vec3::new(0.0, viewport_height, 0.0);
    //let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
    // auto horizontal = vec3(viewport_width, 0, 0);
    // auto vertical = vec3(0, viewport_height, 0);
    // auto lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5, Rc::new(material::Lambertian::new(Color::new(0.7, 0.3, 0.3))))));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0, Rc::new(material::Lambertian::new(Color::new(0.8,0.8,0.0))))));

    world.add(Box::new(Sphere::new(Point3::new(1.0,0.0,-1.0), 0.5, Rc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2))))));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,0.0,-1.0), 0.5, Rc::new(material::Metal::new(Color::new(0.8,0.8,0.8))))));

    let cam = utils::camera::camera::Camera::new();

    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush()?;
        let mut i = 0;
        while i < IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let mut s = 0;
            while s < SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::thread_rng().gen_range(0.0, 1.0))/ (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::thread_rng().gen_range(0.0, 1.0))/ (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
                s += 1;
            }
            utils::color::write_color(io::stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
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

