use std::{collections::HashSet, time::Instant};

use nalgebra::{Matrix4, Perspective3, Vector3};
use sdl2::{event::Event, keyboard::Keycode, sys::SDL_HapticLeftRight};

use crate::{
    giraffeics::{
        camera::Camera,
        render::{
            light::Light,
            material::{Material, MaterialProperties},
            mesh::Mesh,
            mesh_gen::gen_sphere,
            object::Object,
        },
        shader::ShaderProgram,
        vao::VertexArrayObject,
    },
    ui::UI,
};

mod giraffeics;
mod ui;
mod window;

const VERT_SHADER: &str = include_str!("../res/shaders/transform_project.vert");
const FRAG_SHADER: &str = include_str!("../res/shaders/light.frag");

fn main() -> Result<(), String> {
    let (window, mut event_pump, _ctx) = window::init_window()?;
    let mut ui = UI::init(&window);

    let mut keys = HashSet::new();
    let mut camera = Camera::default();

    let vao = VertexArrayObject::new()?;
    vao.bind();

    let shader_program =
        ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER)?;

    let (vertices, indices) = gen_sphere(1., 32, 32);
    let mut object = Object {
        mesh: Mesh::new(&vao, vertices, indices)?,
        material: Material::new(MaterialProperties::EMERALD, &shader_program),
        transform: Matrix4::identity(),
    };

    let mut light = Light {
        position: Vector3::new(2., 2., 2.),
        ambient: Vector3::repeat(1.),
        diffuse: Vector3::repeat(1.),
        specular: Vector3::repeat(1.),
        shader_program: &shader_program,
    };

    let mut proj = Perspective3::new(
        window::WIDTH as f32 / window::HEIGHT as f32,
        camera.get_zoom().to_radians(),
        0.1,
        100.,
    );

    let mut start = Instant::now();
    let mut delta = 0.;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Q => break 'running,
                    Keycode::L => giraffeics::toggle_polygon_mode(),
                    _ => {
                        keys.insert(keycode);
                    }
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    keys.remove(&keycode);
                }
                _ => ui.process_input(event),
            }
        }

        camera.handle_keys(&keys, delta);
        proj.set_fovy(camera.get_zoom().to_radians());

        giraffeics::clear_color(0.2, 0.3, 0.3, 1.0);

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };

        shader_program.use_program();

        shader_program.set_vec3("view_pos", camera.get_position());
        shader_program.set_mat4("view", camera.get_view().as_ptr());
        shader_program.set_mat4("projection", proj.as_matrix().as_ptr());

        light.setup_shader();

        object.render();

        unsafe { gl::Disable(gl::DEPTH_TEST) };

        ui.ctx.begin_pass(ui.egui_state.input.take());

        egui::Window::new("Controls")
            .resizable(false)
            .max_width(120.)
            .show(&ui.ctx, |ui| {
                ui.heading("Light");
                ui::controls::custom_light(ui, &mut light);
                ui.heading("Material");
                ui::controls::presets(ui, &mut object);
                ui::controls::custom_material(ui, &mut object);
            });

        let full_output = ui.ctx.end_pass();
        ui.handle_output(full_output);

        unsafe { gl::Enable(gl::DEPTH_TEST) };

        window.gl_swap_window();

        delta = start.elapsed().as_secs_f32();
        start = Instant::now();
    }

    Ok(())
}
