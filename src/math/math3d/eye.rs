use crate::math::{matrix4x4::Matrix4x4, vector3::Vector3};

pub trait EyeBase {
    fn set_aspect_ratio(&mut self, aspect_ratio: f32);
    fn update_projection(&mut self);
    fn set_pitch(&mut self, pitch: f32);
    fn set_yaw(&mut self, yaw: f32);
    fn set_near(&mut self, near: f32);
    fn set_far(&mut self, far: f32);
    fn set_fov(&mut self, fov: f32);
    fn set_up(&mut self, up: Vector3);
    fn set_front(&mut self, front: Vector3);
}

pub struct Eye {
    pub pitch: f32,
    pub yaw: f32,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub up: Vector3,
    pub front: Vector3,
    pub projection: Matrix4x4,
    pub aspect_ratio: f32,
}

impl Eye {
    pub fn new(projection: Matrix4x4, aspect_ratio: f32) -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            near: 0.01,
            far: 100.0,
            fov: 45.0,
            up: crate::math::vector3::UNIT_Y,
            front: crate::math::vector3::UNIT_Z,
            projection,
            aspect_ratio,
        }
    }
}

impl EyeBase for Eye {
    fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }
    fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
    }
    fn set_near(&mut self, near: f32) {
        self.near = near;
    }
    fn set_far(&mut self, far: f32) {
        self.far = far;
    }
    fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
    fn set_up(&mut self, up: Vector3) {
        self.up = up;
    }
    fn set_front(&mut self, front: Vector3) {
        self.front = front;
    }
    fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
    fn update_projection(&mut self) {
        self.projection =
            Matrix4x4::create_perspective_fov(self.fov, self.aspect_ratio, self.near, self.far);
    }
}
