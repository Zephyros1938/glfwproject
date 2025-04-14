#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    u8(u8),
    i8(i8),
    u16(u16),
    i16(i16),
    u32(u32),
    i32(i32),
    u64(u64),
    i64(i64),
    f32(f32),
    f64(f64),
}

impl Number {
    pub fn Get(n: impl 'static + PartialEq + Clone + Into<Number>) -> Self {
        n.into()
    }
}
