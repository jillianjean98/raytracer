use std::ops::{*};
use std::io::{self, Write};

pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {
            e: [e1, e2, e3]
        }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] + other.e[0], 
                self.e[1] + other.e[1], 
                self.e[2] + other.e[2]]
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e = [self.e[0]*t, self.e[1]*t, self.e[2]*t];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.e = [self.e[0]/t, self.e[1]/t, self.e[2]/t];
    }
}

// type aliases
type Point3 = Vec3; // 3D point
type Color = Vec3; // RGB color

// Vec3 Utility Functions
fn vec_write(v: Vec3) -> io::Result<()> {
    io::stdout().write_fmt(format_args!("{} {} {}", v.e[0], v.e[1], v.e[2]))?;
    Ok(())
}
// inline std::ostream& operator<<(std::ostream &out, const vec3 &v) {
//     return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
// }

impl Add for Vec3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]
        }
    }
}
// vec * vec
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    
    fn mul(self, other: Vec3) -> Self {
        Self {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
        }
    }
}

// vec * value
impl Mul<f64> for Vec3 {
    type Output = Self;
    
    fn mul(self, t: f64) -> Self {
        Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]
        }
    }
}

// value * vec
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, v: Vec3) -> Vec3 {
       v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        (1.0/t) * self
    }
}

fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] +
    u.e[1] * v.e[1] +
    u.e[2] * v.e[2]
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: [u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]]
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.length();
    v / l
}