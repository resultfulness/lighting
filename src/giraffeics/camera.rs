use std::collections::HashSet;

use nalgebra::{Matrix4, Point3, Vector3};
use sdl2::keyboard::Keycode;

pub struct Camera {
    position: Vector3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
    world_up: Vector3<f32>,

    yaw: f32,
    pitch: f32,

    movement_speed: f32,
    look_sensitivity: f32,
    zoom_speed: f32,
    zoom: f32,
}

impl Camera {
    pub fn new(
        position: Vector3<f32>,
        up: Vector3<f32>,
        yaw: f32,
        pitch: f32,
        movement_speed: f32,
        look_sensitivity: f32,
        zoom_speed: f32,
        zoom: f32,
    ) -> Self {
        let front = -Vector3::z();
        let mut cam = Self {
            position,
            front,
            up,
            right: front.cross(&up).normalize(),
            world_up: up,
            yaw,
            pitch,
            movement_speed,
            look_sensitivity,
            zoom_speed,
            zoom,
        };
        cam.update_vectors();
        cam
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.position + self.front),
            &self.up,
        )
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn handle_keys(&mut self, keys: &HashSet<Keycode>, delta: f32) {
        for key in keys {
            match *key {
                Keycode::W => self.move_forward(delta),
                Keycode::S => self.move_backward(delta),
                Keycode::A => self.move_left(delta),
                Keycode::D => self.move_right(delta),
                Keycode::UP => self.look_up(delta),
                Keycode::DOWN => self.look_down(delta),
                Keycode::LEFT => self.look_left(delta),
                Keycode::RIGHT => self.look_right(delta),
                Keycode::EQUALS => self.zoom_in(delta),
                Keycode::MINUS => self.zoom_out(delta),
                _ => {}
            }
        }
    }

    fn update_vectors(&mut self) {
        self.front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }

    fn move_forward(&mut self, delta: f32) {
        self.position += self.front * self.movement_speed * delta;
    }

    fn move_backward(&mut self, delta: f32) {
        self.position -= self.front * self.movement_speed * delta;
    }

    fn move_left(&mut self, delta: f32) {
        self.position -= self.right * self.movement_speed * delta;
    }

    fn move_right(&mut self, delta: f32) {
        self.position += self.right * self.movement_speed * delta;
    }

    fn constrain_pitch(&mut self) {
        self.pitch = self.pitch.clamp(-89.0, 89.0);
    }

    fn look_up(&mut self, delta: f32) {
        self.pitch += self.look_sensitivity * delta;
        self.constrain_pitch();
        self.update_vectors();
    }

    fn look_down(&mut self, delta: f32) {
        self.pitch -= self.look_sensitivity * delta;
        self.constrain_pitch();
        self.update_vectors();
    }

    fn look_left(&mut self, delta: f32) {
        self.yaw -= self.look_sensitivity * delta;
        self.update_vectors();
    }

    fn look_right(&mut self, delta: f32) {
        self.yaw += self.look_sensitivity * delta;
        self.update_vectors();
    }

    fn zoom_in(&mut self, delta: f32) {
        self.zoom -= self.zoom_speed * delta;
    }

    fn zoom_out(&mut self, delta: f32) {
        self.zoom += self.zoom_speed * delta;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vector3::new(0., 0., 3.),
            Vector3::y(),
            -90.,
            0.,
            2.5,
            75.,
            10.,
            45.,
        )
    }
}
