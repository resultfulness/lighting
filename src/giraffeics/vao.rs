use gl::types::GLuint;

pub struct VAO(GLuint);

impl VAO {
    pub fn new() -> Result<Self, String> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        vao.ne(&0)
            .then_some(Self(vao))
            .ok_or("couldn't allocate vao".into())
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.0) };
    }
}
