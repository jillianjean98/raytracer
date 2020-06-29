pub mod vec3;
pub mod color;
pub mod ray;
pub mod camera;

pub mod utils {
    pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
}