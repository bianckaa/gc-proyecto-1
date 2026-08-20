pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point_color(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for j in y..y + h {
            for i in x..x + w {
                self.point_color(i, j, color);
            }
        }
    }

    pub fn rect_border(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for i in x..x + w {
            self.point_color(i, y, color);
            self.point_color(i, y + h.saturating_sub(1), color);
        }
        for j in y..y + h {
            self.point_color(x, j, color);
            self.point_color(x + w.saturating_sub(1), j, color);
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }
}

pub fn shade(color: u32, factor: f32) -> u32 {
    let f = factor.clamp(0.0, 1.0);
    let r = ((color >> 16) & 0xFF) as f32 * f;
    let g = ((color >> 8) & 0xFF) as f32 * f;
    let b = (color & 0xFF) as f32 * f;
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
