use std::io::{self, Write};
#[path = "vec3.rs"] mod vec3;

pub type Color = vec3::Vec3; // RGB color

pub fn write_color(mut output_writer: impl Write, pixel_color: Color) -> io::Result<()> {
    output_writer.write_fmt(format_args!("{} {} {}\n", 
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32))?;
    Ok(())
}
// void write_color(std::ostream &out, color pixel_color) {
//     // Write the translated [0,255] value of each color component.
//     out << static_cast<int>(255.999 * pixel_color.x()) << ' '
//         << static_cast<int>(255.999 * pixel_color.y()) << ' '
//         << static_cast<int>(255.999 * pixel_color.z()) << '\n';
// }