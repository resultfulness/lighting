use std::collections::HashSet;

use nalgebra::{Matrix4, Point3, Vector3};
use sdl2::keyboard::Keycode;

pub struct OrbitCamera {
    radius: f32,
    position: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,

    yaw: f32,
    pitch: f32,

    look_sensitivity: f32,
    zoom_speed: f32,
    zoom: f32,
}

impl OrbitCamera {
    pub fn new(
        radius: f32,
        target: Vector3<f32>,
        up: Vector3<f32>,
        yaw: f32,
        pitch: f32,
        look_sensitivity: f32,
        zoom_speed: f32,
        zoom: f32,
    ) -> Self {
        let mut cam = Self {
            radius,
            target,
            position: Vector3::new(0., 0., -radius),
            up,
            yaw,
            pitch,
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
            &Point3::from(self.target),
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
                Keycode::W => self.orbit_up(delta),
                Keycode::S => self.orbit_down(delta),
                Keycode::A => self.orbit_left(delta),
                Keycode::D => self.orbit_right(delta),
                Keycode::EQUALS => self.zoom_in(delta),
                Keycode::MINUS => self.zoom_out(delta),
                _ => {}
            }
        }
    }

    fn update_vectors(&mut self) {
        let v = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.position = self.target + v * -self.radius;
    }

    fn constrain_pitch(&mut self) {
        self.pitch = self.pitch.clamp(-89.0, 89.0);
    }

    fn orbit_up(&mut self, delta: f32) {
        self.pitch -= self.look_sensitivity * delta;
        self.constrain_pitch();
        self.update_vectors();
    }

    fn orbit_down(&mut self, delta: f32) {
        self.pitch += self.look_sensitivity * delta;
        self.constrain_pitch();
        self.update_vectors();
    }

    fn orbit_left(&mut self, delta: f32) {
        self.yaw += self.look_sensitivity * delta;
        self.update_vectors();
    }

    fn orbit_right(&mut self, delta: f32) {
        self.yaw -= self.look_sensitivity * delta;
        self.update_vectors();
    }

    fn zoom_in(&mut self, delta: f32) {
        self.zoom -= self.zoom_speed * delta;
    }

    fn zoom_out(&mut self, delta: f32) {
        self.zoom += self.zoom_speed * delta;
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new(3., Vector3::zeros(), Vector3::y(), -90., 0., 75., 10., 45.)
    }
}
