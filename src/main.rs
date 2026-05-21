use std::{collections::HashSet, time::Instant};

use egui_sdl2_gl::{
    DpiScaling, EguiStateHandler, ShaderVersion, painter::Painter,
};
use nalgebra::{Matrix4, Perspective3, Vector3};
use sdl2::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    video::{GLContext, SwapInterval, Window},
};

use crate::giraffeics::{
    camera::Camera,
    render::{
        material::Material, mesh::Mesh, mesh_gen::gen_sphere, object::Object,
    },
    shader::ShaderProgram,
    vao::VertexArrayObject,
};

mod giraffeics;

const WINDOW_W: u32 = 800;
const WINDOW_H: u32 = 600;

const VERT_SHADER: &str = include_str!("../res/shaders/transform_project.vert");
const FRAG_SHADER: &str = include_str!("../res/shaders/light.frag");

fn init_window() -> Result<(Window, EventPump, GLContext), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_multisample_samples(8);

    let window = video_subsystem
        .window("lighting", WINDOW_W, WINDOW_H)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context()?;

    let event_pump = sdl_context.event_pump()?;

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::VSync)?;

    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    unsafe { gl::Enable(gl::DEPTH_TEST) };

    Ok((window, event_pump, _ctx))
}

fn init_egui(window: &Window) -> (Painter, EguiStateHandler, egui::Context) {
    let (painter, egui_state) = egui_sdl2_gl::with_sdl2(
        &window,
        ShaderVersion::Default,
        DpiScaling::Default,
    );
    let egui_ctx = egui::Context::default();

    (painter, egui_state, egui_ctx)
}

fn main() -> Result<(), String> {
    let (window, mut event_pump, _ctx) = init_window()?;
    let (mut painter, mut egui_state, egui_ctx) = init_egui(&window);

    let mut keys = HashSet::new();
    let mut camera = Camera::default();

    let vao = VertexArrayObject::new()?;
    vao.bind();

    let shader_program =
        ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER)?;

    let (vertices, indices) = gen_sphere(1., 32, 32);
    let object = Object {
        mesh: Mesh::new(&vao, vertices, indices)?,
        material: Material {
            shader_program: &shader_program,
            ambient: Vector3::new(0.25, 0.25, 0.25),
            diffuse: Vector3::new(0.4, 0.4, 0.4),
            specular: Vector3::new(0.774597, 0.774597, 0.774597),
            shininess: 32.0,
        },
        transform: Matrix4::identity(),
    };

    shader_program.use_program();

    shader_program.set_vec3("light.pos", Vector3::new(2., 2., 2.));
    shader_program.set_vec3("light.ambient", Vector3::new(1., 1., 1.));
    shader_program.set_vec3("light.diffuse", Vector3::new(1., 1., 1.));
    shader_program.set_vec3("light.specular", Vector3::new(1., 1., 1.));

    let mut proj = Perspective3::new(
        WINDOW_W as f32 / WINDOW_H as f32,
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
                _ => {
                    egui_state.process_input(&window, event, &mut painter);
                }
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

        object.render();

        unsafe { gl::Disable(gl::DEPTH_TEST) };

        egui_ctx.begin_pass(egui_state.input.take());

        egui::Window::new("Controls").show(&egui_ctx, |ui| {});

        let full_output = egui_ctx.end_pass();
        egui_state.process_output(&window, &full_output.platform_output);
        let paint_jobs = egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        painter.paint_jobs(None, full_output.textures_delta, paint_jobs);

        unsafe { gl::Enable(gl::DEPTH_TEST) };

        window.gl_swap_window();

        delta = start.elapsed().as_secs_f32();
        start = Instant::now();
    }

    Ok(())
}
