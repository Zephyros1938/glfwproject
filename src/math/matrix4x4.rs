use super::vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Matrix4x4  {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new() -> Self {
        Matrix4x4 { m: [[0.0; 4]; 4] }
    }

    pub fn identity() -> Self {
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.m[3][0] += x;
        self.m[3][1] += y;
        self.m[3][2] += z;
    }
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.m[0][0] *= x;
        self.m[1][1] *= y;
        self.m[2][2] *= z;
    }
    pub fn create_perspective_fov(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        // let f = 1.0 / (fov / 2.0).tan();
        let mut m = Matrix4x4::identity();
        // m.m[0][0] = f / aspect_ratio;
        // m.m[1][1] = f;
        // m.m[2][2] = (far) / (far - near);
        // m.m[3][2] = -((near * far) - (far - near));
        // m.m[3][3] = 0.0;
        // m
        let height = 1.0 / f32::tan(fov * 0.5f32);
        let width = height / aspect_ratio;
        let range = if far == f32::INFINITY {
            -1.0f32
        } else {
            far / (near - far)
        };
        m.m[0] = [width, 0.0, 0.0, 0.0];
        m.m[1] = [0.0, height, 0.0, 0.0];
        m.m[2] = [0.0, 0.0, range, -1.0];
        m.m[3] = [0.0, 0.0, range * near, 0.0];
        m
    }

    pub fn create_translation(v1: Vector3) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [v1.x, v1.y, v1.z, 1.0],
            ],
        }
    }

    pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let forward = Vector3::normalize(target.x - eye.x, target.y - eye.y, target.z - eye.z);

        let right = Vector3::normalize_v3(Vector3::cross(up, forward));

        let true_up = Vector3::cross(forward, right);

        let rotation = Matrix4x4 {
            m: [
                [right.x, true_up.x, -forward.x, 0f32],
                [right.y, true_up.y, -forward.y, 0f32],
                [right.z, true_up.z, -forward.z, 0f32],
                [0f32, 0f32, 0f32, 1f32],
            ],
        };

        let translation = Self::create_translation(-eye);

        translation * rotation
    }

    pub fn create_rotation_x(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, s, 0.0],
                [0.0, -s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn create_rotation_y(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();
        Self {
            m: [
                [c, 0.0, -s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn create_rotation_z(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();
        Self {
            m: [
                [c, s, 0.0, 0.0],
                [-s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.m.as_ptr() as *const f32
    }

    pub fn flatten(self) -> [f32; 16] {
        [
            self.m[0][0],
            self.m[0][1],
            self.m[0][2],
            self.m[0][3],
            self.m[1][0],
            self.m[1][1],
            self.m[1][2],
            self.m[1][3],
            self.m[2][0],
            self.m[2][1],
            self.m[2][2],
            self.m[2][3],
            self.m[3][0],
            self.m[3][1],
            self.m[3][2],
            self.m[3][3],
        ]
    }
}

impl std::ops::Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] + other.m[i][j];
            }
        }
        result
    }
}

impl std::ops::Add<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: f32) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] + other;
            }
        }
        result
    }
}

impl std::ops::Add<Vector3> for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: Vector3) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] + other.x;
            }
        }
        result
    }
}

impl std::ops::Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] - other.m[i][j];
            }
        }
        result
    }
}

impl std::ops::Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[i][k] * other.m[k][j];
                }
                result.m[i][j] = sum;
            }
        }
        result
    }
}

impl std::ops::Mul<Vector3> for Matrix4x4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        let mut result = Vector3::new();
        result.x =
            self.m[0][0] * other.x + self.m[1][0] * other.y + self.m[2][0] * other.z + self.m[3][0];
        result.y =
            self.m[0][1] * other.x + self.m[1][1] * other.y + self.m[2][1] * other.z + self.m[3][1];
        result.z =
            self.m[0][2] * other.x + self.m[1][2] * other.y + self.m[2][2] * other.z + self.m[3][2];
        result
    }
}

impl std::ops::Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: f32) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] * other;
            }
        }
        result
    }
}

impl std::ops::Div for Matrix4x4 {
    type Output = Matrix4x4;

    fn div(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][j] / other.m[i][j];
            }
        }
        result
    }
}

impl std::ops::Neg for Matrix4x4 {
    type Output = Matrix4x4;

    fn neg(self) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = -self.m[i][j];
            }
        }
        result
    }
}

impl std::ops::AddAssign for Matrix4x4 {
    fn add_assign(&mut self, other: Matrix4x4) {
        *self = *self + other;
    }
}

impl std::ops::SubAssign for Matrix4x4 {
    fn sub_assign(&mut self, other: Matrix4x4) {
        *self = *self - other;
    }
}

impl std::ops::MulAssign for Matrix4x4 {
    fn mul_assign(&mut self, other: Matrix4x4) {
        *self = *self * other;
    }
}

impl std::ops::DivAssign for Matrix4x4 {
    fn div_assign(&mut self, other: Matrix4x4) {
        *self = *self / other;
    }
}

impl std::ops::Index<usize> for Matrix4x4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &[f32; 4] {
        &self.m[index]
    }
}

impl std::ops::IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut [f32; 4] {
        &mut self.m[index]
    }
}

impl std::fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix4x4 {{ m: [[{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}]] }}",
            self.m[0][0],
            self.m[0][1],
            self.m[0][2],
            self.m[0][3],
            self.m[1][0],
            self.m[1][1],
            self.m[1][2],
            self.m[1][3],
            self.m[2][0],
            self.m[2][1],
            self.m[2][2],
            self.m[2][3],
            self.m[3][0],
            self.m[3][1],
            self.m[3][2],
            self.m[3][3]
        )
    }
}
