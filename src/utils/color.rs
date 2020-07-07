
// pub mod color {
    
use std::io::{self, Write};
use crate::utils::vec3::color::Color;
use crate::utils::utils;

pub fn write_color(mut output_writer: impl Write, pixel_color: Color, samples_per_pixel: i32) -> io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;
    
    output_writer.write_fmt(format_args!("{} {} {}\n", 
        (256.0 * utils::clamp(r.sqrt(), 0.0, 0.999)) as i32,
        (256.0 * utils::clamp(g.sqrt(), 0.0, 0.999)) as i32,
        (256.0 * utils::clamp(b.sqrt(), 0.0, 0.999)) as i32))?;
    Ok(())
}

//}

//pub type Color = vec3::Color; // RGB color

