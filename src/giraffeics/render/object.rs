use nalgebra::Matrix4;

use crate::giraffeics::render::{material::Material, mesh::Mesh};

pub struct Object<'a> {
    pub mesh: Mesh<'a>,
    pub material: Material<'a>,
    pub transform: Matrix4<f32>,
}

impl Object<'_> {
    pub fn render(&self) {
        self.material.bind();
        self.material
            .shader_program
            .set_mat4("model", self.transform.as_ptr());
        self.mesh.draw();
    }
}
