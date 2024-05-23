
use ogl33::*;
pub struct LearnLib {}
impl LearnLib
{
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32)
    {
        unsafe { glClearColor(r, g, b, a)}
    }
}
pub struct VertexArray(pub GLuint);
impl VertexArray
{
    pub fn new() -> Option<Self>
    {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self(vao))
        } else
        {
            None
        }
    }

    pub fn bind(&self) {
        unsafe { glBindVertexArray(self.0) }
    }

    pub fn clear_binding() {
        unsafe { glBindVertexArray(0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = GL_ARRAY_BUFFER as isize,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);
impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    pub fn bind(&self, ty: BufferType) {
        unsafe { glBindBuffer (ty as GLenum, self.0) }
    }

    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer (ty as GLenum, 0) }
    }   
}
pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        glBufferData(
            ty as GLenum,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}