
// pub mod color {
    
use std::io::{self, Write};
use crate::vectors::vec3::color::Color;

pub fn write_color(mut output_writer: impl Write, pixel_color: Color) -> io::Result<()> {
    output_writer.write_fmt(format_args!("{} {} {}\n", 
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32))?;
    Ok(())
}

//}

//pub type Color = vec3::Color; // RGB color

