use crate::framebuffer::Framebuffer;

pub fn point(x: usize, y: usize, color: u32, framebuffer: &mut Framebuffer) {
    framebuffer.set_point(x, y, color);
}
