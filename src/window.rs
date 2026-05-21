use sdl2::{
    EventPump,
    video::{GLContext, SwapInterval, Window},
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub fn init_window() -> Result<(Window, EventPump, GLContext), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_multisample_samples(8);

    let window = video_subsystem
        .window("lighting", WIDTH, HEIGHT)
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
