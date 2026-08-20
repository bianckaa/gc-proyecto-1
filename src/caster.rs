use crate::maze::{is_walkable, Maze};
use crate::player::Player;

const STEP: f32 = 1.0;
const MAX_DISTANCE: f32 = 2200.0;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tex_x: f32,
}

pub fn cast_ray(maze: &Maze, player: &Player, a: f32, block_size: usize) -> Intersect {
    let cos = a.cos();
    let sin = a.sin();
    let block = block_size as f32;
    let mut d = 0.0;

    while d < MAX_DISTANCE {
        let x = player.pos.x + d * cos;
        let y = player.pos.y + d * sin;

        if x < 0.0 || y < 0.0 {
            break;
        }

        let i = x as usize / block_size;
        let j = y as usize / block_size;

        let cell = match maze.get(j).and_then(|row| row.get(i)) {
            Some(&cell) => cell,
            None => break,
        };

        if !is_walkable(cell) {
            let hit_x = x - (i * block_size) as f32;
            let hit_y = y - (j * block_size) as f32;
            let along = if hit_x < 2.0 || hit_x > block - 2.0 {
                hit_y
            } else {
                hit_x
            };
            return Intersect {
                distance: d.max(0.001),
                impact: cell,
                tex_x: (along / block).clamp(0.0, 0.999),
            };
        }

        d += STEP;
    }

    Intersect {
        distance: MAX_DISTANCE,
        impact: ' ',
        tex_x: 0.0,
    }
}
