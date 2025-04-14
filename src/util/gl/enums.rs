mod draw_mode;
pub use draw_mode::DrawMode;

pub trait DataType {
    fn value(&self) -> u32;
}

impl DataType for f32 {
    fn value(&self) -> u32 {
        gl::FLOAT
    }
}

impl DataType for i32 {
    fn value(&self) -> u32 {
        gl::INT
    }
}

impl DataType for u32 {
    fn value(&self) -> u32 {
        gl::UNSIGNED_INT
    }
}

impl DataType for u8 {
    fn value(&self) -> u32 {
        gl::UNSIGNED_BYTE
    }
}

impl DataType for i8 {
    fn value(&self) -> u32 {
        gl::BYTE
    }
}

impl DataType for i16 {
    fn value(&self) -> u32 {
        gl::SHORT
    }
}

impl DataType for u16 {
    fn value(&self) -> u32 {
        gl::UNSIGNED_SHORT
    }
}

impl DataType for i64 {
    fn value(&self) -> u32 {
        gl::INT
    }
}

impl DataType for u64 {
    fn value(&self) -> u32 {
        gl::UNSIGNED_INT
    }
}

impl DataType for f64 {
    fn value(&self) -> u32 {
        gl::DOUBLE
    }
}

impl DataType for bool {
    fn value(&self) -> u32 {
        gl::BOOL
    }
}

impl DataType for Vec<f32> {
    fn value(&self) -> u32 {
        gl::FLOAT
    }
}

impl DataType for Vec<i32> {
    fn value(&self) -> u32 {
        gl::INT
    }
}

impl DataType for Vec<u32> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_INT
    }
}

impl DataType for Vec<u8> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_BYTE
    }
}

impl DataType for Vec<i8> {
    fn value(&self) -> u32 {
        gl::BYTE
    }
}

impl DataType for Vec<i16> {
    fn value(&self) -> u32 {
        gl::SHORT
    }
}

impl DataType for Vec<u16> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_SHORT
    }
}

impl DataType for Vec<i64> {
    fn value(&self) -> u32 {
        gl::INT
    }
}

impl DataType for Vec<u64> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_INT
    }
}

impl DataType for Vec<f64> {
    fn value(&self) -> u32 {
        gl::DOUBLE
    }
}

impl DataType for Vec<bool> {
    fn value(&self) -> u32 {
        gl::BOOL
    }
}

impl DataType for Vec<Vec<f32>> {
    fn value(&self) -> u32 {
        gl::FLOAT
    }
}

impl DataType for Vec<Vec<i32>> {
    fn value(&self) -> u32 {
        gl::INT
    }
}

impl DataType for Vec<Vec<u32>> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_INT
    }
}

impl DataType for Vec<Vec<u8>> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_BYTE
    }
}

impl DataType for Vec<Vec<i8>> {
    fn value(&self) -> u32 {
        gl::BYTE
    }
}

impl DataType for Vec<Vec<i16>> {
    fn value(&self) -> u32 {
        gl::SHORT
    }
}

impl DataType for Vec<Vec<u16>> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_SHORT
    }
}

impl DataType for Vec<Vec<i64>> {
    fn value(&self) -> u32 {
        gl::INT
    }
}

impl DataType for Vec<Vec<u64>> {
    fn value(&self) -> u32 {
        gl::UNSIGNED_INT
    }
}

impl DataType for Vec<Vec<f64>> {
    fn value(&self) -> u32 {
        gl::DOUBLE
    }
}

impl DataType for Vec<Vec<bool>> {
    fn value(&self) -> u32 {
        gl::BOOL
    }
}
