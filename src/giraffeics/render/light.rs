use nalgebra::Vector3;

use crate::giraffeics::shader::ShaderProgram;

pub struct Light<'a> {
    pub position: Vector3<f32>,
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shader_program: &'a ShaderProgram,
}

impl<'a> Light<'a> {
    pub fn new(
        position: Vector3<f32>,
        ambient: Vector3<f32>,
        diffuse: Vector3<f32>,
        specular: Vector3<f32>,
        shader_program: &'a ShaderProgram,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            shader_program,
        }
    }

    pub fn setup_shader(&self) {
        self.shader_program.use_program();
        self.shader_program.set_vec3("light.pos", self.position);
        self.shader_program.set_vec3("light.ambient", self.ambient);
        self.shader_program.set_vec3("light.diffuse", self.diffuse);
        self.shader_program
            .set_vec3("light.specular", self.specular);
    }
}
