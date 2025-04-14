#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const ZERO: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const ONE: Vector3 = Vector3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
pub const UNIT_X: Vector3 = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const UNIT_Y: Vector3 = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const UNIT_Z: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl Vector3 {
    pub const fn new() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub const fn from(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
    pub const fn cross(v1: Vector3, v2: Vector3) -> Self {
        Vector3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
    pub fn normalize(x: f32, y: f32, z: f32) -> Self {
        let length = ((x * x) + (y * y) + (z * z)).sqrt();
        if length == 0.0 {
            return Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        Vector3 {
            x: x / length,
            y: y / length,
            z: z / length,
        }
    }
    pub fn normalize_v3(v: Vector3) -> Self {
        let length = ((v.x * v.x) + (v.y * v.y) + (v.z * v.z)).sqrt();
        if length == 0.0 {
            return Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        Vector3 {
            x: v.x / length,
            y: v.y / length,
            z: v.z / length,
        }
    }
    pub fn normal(self) -> Self {
        Vector3::normalize_v3(self)
    }
    pub fn translate(&self, translation: Vector3) -> Self {
        Vector3 {
            x: self.x + translation.x,
            y: self.y + translation.y,
            z: self.z + translation.z,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Div for Vector3 {
    type Output = Vector3;
    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl std::ops::DivAssign for Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vector3 {{ x: {}, y: {}, z: {} }}",
            self.x, self.y, self.z
        )
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector3 {}

impl std::hash::Hash for Vector3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}
