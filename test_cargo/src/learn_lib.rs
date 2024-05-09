
use ogl33::*;
pub struct LearnLib {}
impl LearnLib
{
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32)
    {
        unsafe { glClearColor(r, g, b, a)}
    }
}