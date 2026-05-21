pub mod camera;
pub mod buffer_object;
pub mod orbit_camera;
pub mod vao;
pub mod shader;
pub mod render;

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { gl::ClearColor(red, green, blue, alpha) }
}

pub fn toggle_polygon_mode() {
    unsafe {
        let mut polygon_mode = 0;
        gl::GetIntegerv(gl::POLYGON_MODE, &mut polygon_mode);
        if polygon_mode == gl::LINE as i32 {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }
}
