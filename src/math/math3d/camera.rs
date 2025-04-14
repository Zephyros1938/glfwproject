use super::eye::{Eye, EyeBase};
use crate::math::{
    mathhelper, matrix4x4,
    vector3::{self, Vector3},
};

pub struct Camera {
    pub eye: Eye,
    pub position: vector3::Vector3,
    pub last_position: [f32; 2],
    pub speed: f32,
    pub sensitivity: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    first_move: bool,
}

impl EyeBase for Camera {
    fn set_pitch(&mut self, pitch: f32) {
        self.eye.set_pitch(pitch);
    }

    fn set_yaw(&mut self, yaw: f32) {
        self.eye.set_yaw(yaw);
    }

    fn set_near(&mut self, near: f32) {
        self.eye.set_near(near);
    }

    fn set_far(&mut self, far: f32) {
        self.eye.set_far(far);
    }

    fn set_fov(&mut self, fov: f32) {
        self.eye.set_fov(fov);
    }

    fn set_up(&mut self, up: vector3::Vector3) {
        self.eye.set_up(up);
    }

    fn set_front(&mut self, front: vector3::Vector3) {
        self.eye.set_front(front);
    }

    fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.eye.set_aspect_ratio(aspect_ratio);
        self.eye.update_projection();
    }

    fn update_projection(&mut self) {
        self.eye.update_projection();
    }
}

impl Camera {
    pub fn new(
        pos: vector3::Vector3,
        projection: matrix4x4::Matrix4x4,
        speed: f32,
        sensitivity: f32,
        aspect_ratio: f32,
    ) -> Self {
        let eye = Eye::new(projection, aspect_ratio);
        let eyeup = eye.up;
        let eyefront = eye.front;
        Self {
            eye,
            position: pos,
            last_position: [0f32, 0f32],
            speed,
            sensitivity,
            delta_x: 0.0,
            delta_y: 0.0,
            first_move: true,
        }
    }

    pub fn update(&mut self, new_pos: [f32; 2]) {
        if self.first_move {
            self.update_last_pos(new_pos);
            self.first_move = false;
        } else {
            self.delta_x = new_pos[0] - self.last_position[0];
            self.delta_y = new_pos[1] - self.last_position[1];
            self.update_last_pos(new_pos);
            self.eye.yaw -= self.delta_x * self.sensitivity;
            if self.eye.pitch > 89f32 {
                self.eye.pitch += self.delta_y * self.sensitivity;
            } else if self.eye.pitch < -89f32 {
                self.eye.pitch = -89f32;
            } else {
                self.eye.pitch -= self.delta_y * self.sensitivity;
            }
        }
    }

    pub fn update_last_pos(&mut self, new_pos: [f32; 2]) {
        self.last_position = new_pos;
    }

    pub fn rotate(&mut self) {
        self.eye.front.x = mathhelper::degrees_to_radians_f32(self.eye.pitch).cos()
            * mathhelper::degrees_to_radians_f32(self.eye.yaw).cos();
        self.eye.front.y = mathhelper::degrees_to_radians_f32(self.eye.pitch).sin();
        self.eye.front.z = mathhelper::degrees_to_radians_f32(self.eye.pitch).cos()
            * mathhelper::degrees_to_radians_f32(self.eye.yaw).sin();
        self.eye.front = self.eye.front.normal();
    }

    pub fn set_position(&mut self, pos: vector3::Vector3) {
        self.position = pos;
    }

    pub fn get_view(&mut self) -> matrix4x4::Matrix4x4 {
        matrix4x4::Matrix4x4::look_at(self.position, self.position + self.eye.front, self.eye.up)
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    pub fn forward(&mut self, delta: f32) {
        self.position += self.eye.front * delta * self.speed;
    }
    pub fn backward(&mut self, delta: f32) {
        self.position -= self.eye.front * delta * self.speed;
    }
    pub fn right(&mut self, delta: f32) {
        self.position -=
            vector3::Vector3::normalize_v3(Vector3::cross(self.eye.front, self.eye.up))
                * delta
                * self.speed;
    }
    pub fn left(&mut self, delta: f32) {
        self.position +=
            vector3::Vector3::normalize_v3(Vector3::cross(self.eye.front, self.eye.up))
                * delta
                * self.speed;
    }
    pub fn up(&mut self, delta: f32) {
        self.position += self.eye.up * delta * self.speed;
    }
    pub fn down(&mut self, delta: f32) {
        self.position -= self.eye.up * delta * self.speed;
    }
}
