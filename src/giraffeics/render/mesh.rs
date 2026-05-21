use crate::giraffeics::{
    buffer_object::{BufferObject, BufferType},
    vao::VertexArrayObject,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl Vertex {
    pub fn new_from_scalars(
        position_x: f32,
        position_y: f32,
        position_z: f32,
        normal_x: f32,
        normal_y: f32,
        normal_z: f32,
    ) -> Self {
        Self {
            position: [position_x, position_y, position_z],
            normal: [normal_x, normal_y, normal_z],
        }
    }
}

pub struct Mesh<'a> {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vao: &'a VertexArrayObject,
}

impl<'a> Mesh<'a> {
    pub fn new(
        vao: &'a VertexArrayObject,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Result<Self, String> {
        let m = Self {
            vao,
            vertices,
            indices,
        };
        m.setup()?;
        Ok(m)
    }

    fn setup(&self) -> Result<(), String> {
        let vbo = BufferObject::new()?;
        vbo.bind(BufferType::Array);
        vbo.buffer_data(
            bytemuck::cast_slice(&self.vertices),
            BufferType::Array,
            gl::STATIC_DRAW,
        );

        let ebo = BufferObject::new()?;
        ebo.bind(BufferType::ElementArray);
        ebo.buffer_data(
            bytemuck::cast_slice(&self.indices),
            BufferType::ElementArray,
            gl::STATIC_DRAW,
        );

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                size_of::<[f32; 3]>() as *const _,
            );
            gl::EnableVertexAttribArray(1);
        }

        self.vao.bind();

        Ok(())
    }

    pub fn draw(&self) {
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            )
        };
    }
}
