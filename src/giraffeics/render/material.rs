use nalgebra::Vector3;

use crate::giraffeics::shader::ShaderProgram;

pub struct Material<'a> {
    pub shader_program: &'a ShaderProgram,
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Material<'_> {
    pub fn bind(&self) {
        self.shader_program.use_program();

        self.shader_program
            .set_vec3("material.ambient", self.ambient);
        self.shader_program
            .set_vec3("material.diffuse", self.diffuse);
        self.shader_program
            .set_vec3("material.specular", self.specular);
        self.shader_program
            .set_float("material.shininess", self.shininess);
    }
}
