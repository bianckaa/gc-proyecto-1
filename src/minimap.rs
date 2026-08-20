use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::texture::fallback_color;

const CELL: usize = 7;
const MARGIN: usize = 12;

pub fn draw(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    let cols = maze.iter().map(|row| row.len()).max().unwrap_or(0);
    let rows = maze.len();
    if cols == 0 || rows == 0 {
        return;
    }

    let width = cols * CELL + 4;
    let height = rows * CELL + 4;
    let origin_x = framebuffer.width.saturating_sub(width + MARGIN);
    let origin_y = MARGIN;

    framebuffer.fill_rect(origin_x, origin_y, width, height, 0x14140F);

    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            let x = origin_x + 2 + col * CELL;
            let y = origin_y + 2 + row * CELL;
            if cell == 'g' {
                framebuffer.fill_rect(x, y, CELL, CELL, 0x3CE07A);
            } else if cell != ' ' {
                framebuffer.fill_rect(x, y, CELL, CELL, fallback_color(cell));
            }
        }
    }

    framebuffer.rect_border(origin_x, origin_y, width, height, 0xC9B79A);

    let scale = CELL as f32 / block_size as f32;
    let px = origin_x as f32 + 2.0 + player.pos.x * scale;
    let py = origin_y as f32 + 2.0 + player.pos.y * scale;

    for step in 0..12 {
        let d = step as f32;
        let x = px + d * player.a.cos();
        let y = py + d * player.a.sin();
        if x >= 0.0 && y >= 0.0 {
            framebuffer.point_color(x as usize, y as usize, 0xFFE066);
        }
    }

    let dot_x = (px as usize).saturating_sub(1);
    let dot_y = (py as usize).saturating_sub(1);
    framebuffer.fill_rect(dot_x, dot_y, 3, 3, 0xFF4444);
}
