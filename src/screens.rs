use crate::framebuffer::Framebuffer;
use crate::text::{draw_text, draw_text_centered, text_width};

pub struct Level {
    pub name: String,
    pub path: String,
}

fn backdrop(framebuffer: &mut Framebuffer) {
    for y in 0..framebuffer.height {
        let t = y as f32 / framebuffer.height as f32;
        let r = (26.0 + 44.0 * t) as u32;
        let g = (30.0 + 30.0 * t) as u32;
        let b = (22.0 + 16.0 * t) as u32;
        let color = (r << 16) | (g << 8) | b;
        for x in 0..framebuffer.width {
            framebuffer.point_color(x, y, color);
        }
    }
}

fn vine_frame(framebuffer: &mut Framebuffer) {
    let w = framebuffer.width;
    let h = framebuffer.height;
    framebuffer.rect_border(18, 18, w.saturating_sub(36), h.saturating_sub(36), 0x4E8A34);
    framebuffer.rect_border(22, 22, w.saturating_sub(44), h.saturating_sub(44), 0x2F5A22);
}

pub fn draw_welcome(framebuffer: &mut Framebuffer) {
    backdrop(framebuffer);
    vine_frame(framebuffer);
    draw_text_centered(framebuffer, 110, 8, 0xD8E0C0, "MAZE");
    draw_text_centered(framebuffer, 200, 8, 0xB5561F, "RUNNER");
    draw_text_centered(framebuffer, 320, 2, 0x8A8A80, "PROYECTO 1 - RAY CASTER - CC2018 UVG");
    draw_text_centered(framebuffer, 390, 3, 0xFFE066, "ENTER PARA ELEGIR SECTOR");
    draw_text_centered(framebuffer, 440, 2, 0x6E7062, "ESC PARA SALIR");
    draw_text_centered(
        framebuffer,
        510,
        2,
        0x4E8A34,
        "NUNCA TE DETENGAS. NUNCA TE PIERDAS.",
    );
}

pub fn draw_level_select(framebuffer: &mut Framebuffer, levels: &[Level], selected: usize) {
    backdrop(framebuffer);
    vine_frame(framebuffer);
    draw_text_centered(framebuffer, 90, 5, 0xD8E0C0, "ELIGE TU SECTOR");
    draw_text_centered(framebuffer, 160, 2, 0x8A8A80, "FLECHAS ARRIBA/ABAJO - ENTER CONFIRMA");

    for (index, level) in levels.iter().enumerate() {
        let y = 250 + index * 70;
        let label = level.name.to_uppercase();
        let width = text_width(&label, 4);
        let x = framebuffer.width.saturating_sub(width) / 2;
        if index == selected {
            framebuffer.fill_rect(
                x.saturating_sub(24),
                y.saturating_sub(12),
                width + 48,
                52,
                0x2F5A22,
            );
            draw_text(framebuffer, x, y, 4, 0xFFE066, &label);
        } else {
            draw_text(framebuffer, x, y, 4, 0x8A8A80, &label);
        }
    }

    draw_text_centered(framebuffer, 520, 2, 0x6E7062, "ESC PARA VOLVER");
}

pub fn draw_success(framebuffer: &mut Framebuffer, level_name: &str, seconds: f32) {
    backdrop(framebuffer);
    vine_frame(framebuffer);
    draw_text_centered(framebuffer, 110, 6, 0x3CE07A, "META ALCANZADA");
    draw_text_centered(framebuffer, 200, 3, 0xD8E0C0, "ESCAPASTE DEL LABERINTO");
    draw_text_centered(framebuffer, 280, 3, 0xFFE066, &level_name.to_uppercase());
    let tiempo = format!("TIEMPO: {} SEGUNDOS", seconds as u32);
    draw_text_centered(framebuffer, 350, 3, 0x8A8A80, &tiempo);
    draw_text_centered(framebuffer, 440, 2, 0x4E8A34, "ENTER PARA VOLVER AL MENU");
    draw_text_centered(framebuffer, 480, 2, 0x6E7062, "ESC PARA SALIR");
}
