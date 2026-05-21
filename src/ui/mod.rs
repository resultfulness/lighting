pub mod controls;

use egui::FullOutput;
use egui_sdl2_gl::{
    DpiScaling, EguiStateHandler, ShaderVersion, painter::Painter,
};
use sdl2::{event::Event, video::Window};

pub struct UI<'a> {
    painter: Painter,
    pub egui_state: EguiStateHandler,
    pub ctx: egui::Context,
    window: &'a Window,
}

impl<'a> UI<'a> {
    pub fn init(window: &'a Window) -> Self {
        let (painter, egui_state) = egui_sdl2_gl::with_sdl2(
            &window,
            ShaderVersion::Default,
            DpiScaling::Default,
        );
        let ctx = egui::Context::default();

        Self {
            painter,
            egui_state,
            ctx,
            window,
        }
    }

    pub fn process_input(&mut self, event: Event) {
        self.egui_state
            .process_input(self.window, event, &mut self.painter);
    }

    pub fn handle_output(&mut self, full_output: FullOutput) {
        self.egui_state
            .process_output(self.window, &full_output.platform_output);
        let paint_jobs = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        self.painter
            .paint_jobs(None, full_output.textures_delta, paint_jobs);
    }
}
