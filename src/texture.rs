const SIZE: usize = 64;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    pub fn get_pixel(&self, u: f32, v: f32) -> u32 {
        if self.width == 0 || self.height == 0 {
            return 0x808080;
        }
        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);
        self.pixels[y * self.width + x]
    }
}

fn noise(x: usize, y: usize, seed: u32) -> f32 {
    let mut h = x as u32 * 374761393 + y as u32 * 668265263 + seed * 2246822519;
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    ((h ^ (h >> 16)) % 1000) as f32 / 1000.0
}

fn mix(base: u32, factor: f32) -> u32 {
    let f = factor.clamp(0.0, 1.6);
    let r = (((base >> 16) & 0xFF) as f32 * f).min(255.0);
    let g = (((base >> 8) & 0xFF) as f32 * f).min(255.0);
    let b = ((base & 0xFF) as f32 * f).min(255.0);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn stone_texture() -> Texture {
    let mut pixels = vec![0u32; SIZE * SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let row = y / 16;
            let offset = if row % 2 == 0 { 0 } else { 16 };
            let brick_x = (x + offset) % 32;
            let mortar = y % 16 < 2 || brick_x < 2;
            let color = if mortar {
                mix(0x5B5B55, 0.7 + noise(x, y, 1) * 0.2)
            } else {
                mix(0x8A8A80, 0.8 + noise(x / 2, y / 2, 2) * 0.35)
            };
            pixels[y * SIZE + x] = color;
        }
    }
    Texture {
        width: SIZE,
        height: SIZE,
        pixels,
    }
}

fn metal_texture() -> Texture {
    let mut pixels = vec![0u32; SIZE * SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let seam = y % 32 < 2 || x % 32 < 2;
            let rivet_x = x % 32;
            let rivet_y = y % 32;
            let rivet = (rivet_x >= 4 && rivet_x <= 7 && rivet_y >= 4 && rivet_y <= 7)
                || (rivet_x >= 24 && rivet_x <= 27 && rivet_y >= 24 && rivet_y <= 27);
            let rust = noise(x / 3, y / 3, 5);
            let color = if rivet {
                mix(0xC9B79A, 1.0)
            } else if seam {
                mix(0x4A2B18, 0.9)
            } else if rust > 0.62 {
                mix(0xB5561F, 0.75 + rust * 0.4)
            } else {
                mix(0x8A5A34, 0.7 + noise(x, y, 6) * 0.45)
            };
            pixels[y * SIZE + x] = color;
        }
    }
    Texture {
        width: SIZE,
        height: SIZE,
        pixels,
    }
}

fn vine_texture() -> Texture {
    let mut pixels = vec![0u32; SIZE * SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let stone = mix(0x6E7062, 0.75 + noise(x / 2, y / 2, 9) * 0.35);
            let wave = ((y as f32 * 0.25).sin() * 6.0) as i32;
            let stem_a = (x as i32 - (14 + wave)).abs() <= 1;
            let stem_b = (x as i32 - (42 - wave)).abs() <= 1;
            let leaf = noise(x / 2, y / 2, 11) > 0.78
                && ((x as i32 - (14 + wave)).abs() <= 6 || (x as i32 - (42 - wave)).abs() <= 6);
            let color = if stem_a || stem_b {
                mix(0x2F5A22, 0.9 + noise(x, y, 12) * 0.3)
            } else if leaf {
                mix(0x4E8A34, 0.8 + noise(x, y, 13) * 0.45)
            } else {
                stone
            };
            pixels[y * SIZE + x] = color;
        }
    }
    Texture {
        width: SIZE,
        height: SIZE,
        pixels,
    }
}

pub struct TextureSet {
    pub stone: Texture,
    pub metal: Texture,
    pub vine: Texture,
}

impl TextureSet {
    pub fn load() -> Self {
        let stone = stone_texture();
        let metal = metal_texture();
        let vine = vine_texture();
        TextureSet { stone, metal, vine }
    }

    pub fn for_cell(&self, cell: char) -> &Texture {
        match cell {
            '#' => &self.metal,
            '%' => &self.vine,
            _ => &self.stone,
        }
    }
}

pub fn fallback_color(cell: char) -> u32 {
    match cell {
        '#' => 0x8A5A34,
        '%' => 0x4E8A34,
        _ => 0x8A8A80,
    }
}
