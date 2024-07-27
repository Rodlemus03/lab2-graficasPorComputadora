pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        Framebuffer { width, height, buffer }
    }

    pub fn set_point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn clear(&mut self, color: u32) {
        for i in self.buffer.iter_mut() {
            *i = color;
        }
    }
}
