use std::io::{self, Write};
mod vec3;

fn main() -> io::Result<()> {
    let mut v = vec3::Vec3::new(0.0,1.0,0.0);
    println!("{}", v[1]);
    v *= 2.0;
    println!("{}", v[1]);
    // draw ppm image
    // const IMAGE_WIDTH:i32 = 256;
    // const IMAGE_HEIGHT:i32 = 256;

    // io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT))?;

    // let mut j = IMAGE_HEIGHT -1;
    // while j >= 0 {
    //     io::stderr().write_fmt(format_args!("\rScanlines remaining: {}", j))?;
    //     io::stderr().flush()?;
    //     let mut i = 0;
    //     while i < IMAGE_WIDTH {
    //         let r = i as f64 / (IMAGE_WIDTH-1) as f64;
    //         let g = j as f64 / (IMAGE_HEIGHT-1) as f64;
    //         let b = 0.25;
    //         let ir = (255.999 * r) as i32;
    //         let ig = (255.999 * g) as i32;
    //         let ib = (255.999 * b) as i32;
    //         io::stdout().write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
    //         i+=1;
    //     }
    //     j-=1;
    // }
    // io::stderr().write_all(b"\nDone.\n")?;
     Ok(())
}