use gl::types::{GLenum, GLuint};

pub enum BufferType {
    Array = gl::ARRAY_BUFFER as isize,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct BO(GLuint);

impl BO {
    pub fn new() -> Result<Self, String> {
        let mut bo = 0;
        unsafe { gl::GenBuffers(1, &mut bo) };
        bo.ne(&0)
            .then_some(Self(bo))
            .ok_or("couldn't allocate vbo".into())
    }

    pub fn bind(&self, ty: BufferType) {
        unsafe { gl::BindBuffer(ty as GLenum, self.0) };
    }

    pub fn buffer_data(&self, data: &[u8], ty: BufferType, usage: GLenum) {
        unsafe {
            gl::BufferData(
                ty as GLenum,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            );
        }
    }
}
