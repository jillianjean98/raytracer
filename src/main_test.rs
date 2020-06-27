use std::io::{self, Write};
mod color;

// used for setting up ppm and vec3 class, change to main() to run
fn main_tst() -> io::Result<()> {
    // let mut v = vec3::Vec3::new(0.0,1.0,0.0);
    // println!("{}", v[1]);
    // v *= 2.0;
    // println!("{}", v[1]);
    // draw ppm image
    const IMAGE_WIDTH:i32 = 256;
    const IMAGE_HEIGHT:i32 = 256;

    io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT))?;

    let mut j = IMAGE_HEIGHT -1;
    while j >= 0 {
        io::stderr().write_fmt(format_args!("\rScanlines remaining: {}", j))?;
        io::stderr().flush()?;
        let mut i = 0;
        while i < IMAGE_WIDTH {
            let mut pixel_color = color::Color::new(i as f64 / (IMAGE_WIDTH-1) as f64, j as f64 / (IMAGE_HEIGHT-1) as f64, 0.25);
            color::write_color(io::stdout(), pixel_color)?;
            i+=1;
        }
        j-=1;
    }
    io::stderr().write_all(b"\nDone.\n")?;
     Ok(())
}