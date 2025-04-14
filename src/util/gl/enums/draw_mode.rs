#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum DrawMode {
    POINTS = 0,
    TRIANGLES = 100,
    TRIANGLE_STRIP = 101,
    TRIANGLE_FAN = 102,
    LINES = 300,
    LINE_LOOP = 301,
    LINE_STRIP = 302,
}

impl DrawMode {
    pub fn value(&self) -> u32 {
        match self {
            DrawMode::POINTS => gl::POINTS,
            DrawMode::TRIANGLES => gl::TRIANGLES,
            DrawMode::TRIANGLE_STRIP => gl::TRIANGLE_STRIP,
            DrawMode::TRIANGLE_FAN => gl::TRIANGLE_FAN,
            DrawMode::LINES => gl::LINES,
            DrawMode::LINE_LOOP => gl::LINE_LOOP,
            DrawMode::LINE_STRIP => gl::LINE_STRIP,
        }
    }
    pub fn default() -> Self {
        DrawMode::TRIANGLES
    }
}
impl std::fmt::Display for DrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawMode::POINTS => write!(f, "POINTS"),
            DrawMode::TRIANGLES => write!(f, "TRIANGLES"),
            DrawMode::TRIANGLE_STRIP => write!(f, "TRIANGLE_STRIP"),
            DrawMode::TRIANGLE_FAN => write!(f, "TRIANGLE_FAN"),
            DrawMode::LINES => write!(f, "LINES"),
            DrawMode::LINE_LOOP => write!(f, "LINE_LOOP"),
            DrawMode::LINE_STRIP => write!(f, "LINE_STRIP"),
        }
    }
}

impl std::fmt::Debug for DrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawMode::POINTS => write!(f, "DrawMode::POINTS"),
            DrawMode::TRIANGLES => write!(f, "DrawMode::TRIANGLES"),
            DrawMode::TRIANGLE_STRIP => write!(f, "DrawMode::TRIANGLE_STRIP"),
            DrawMode::TRIANGLE_FAN => write!(f, "DrawMode::TRIANGLE_FAN"),
            DrawMode::LINES => write!(f, "DrawMode::LINES"),
            DrawMode::LINE_LOOP => write!(f, "DrawMode::LINE_LOOP"),
            DrawMode::LINE_STRIP => write!(f, "DrawMode::LINE_STRIP"),
        }
    }
}
