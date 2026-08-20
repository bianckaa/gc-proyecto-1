use minifb::{Key, MouseMode, Window};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;

use crate::maze::{cell_at, is_walkable, Maze};

const MOVE_SPEED: f32 = 12.0;
const ROTATION_SPEED: f32 = PI / 24.0;
const MOUSE_SENSITIVITY: f32 = 0.006;
const BODY_RADIUS: f32 = 14.0;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
}

pub struct MouseLook {
    pub last_x: Option<f32>,
}

impl MouseLook {
    pub fn new() -> Self {
        MouseLook { last_x: None }
    }

    pub fn reset(&mut self) {
        self.last_x = None;
    }
}

fn free_at(maze: &Maze, block_size: usize, x: f32, y: f32) -> bool {
    is_walkable(cell_at(maze, block_size, x, y))
}

pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
    mouse: &mut MouseLook,
) -> bool {
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    if let Some((mx, _)) = window.get_mouse_pos(MouseMode::Pass) {
        if let Some(last) = mouse.last_x {
            player.a += (mx - last) * MOUSE_SENSITIVITY;
        }
        mouse.last_x = Some(mx);
    }

    if player.a > 2.0 * PI {
        player.a -= 2.0 * PI;
    }
    if player.a < 0.0 {
        player.a += 2.0 * PI;
    }

    let mut forward = 0.0;
    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        forward += MOVE_SPEED;
    }
    if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        forward -= MOVE_SPEED;
    }

    if forward == 0.0 {
        return false;
    }

    let dx = forward * player.a.cos();
    let dy = forward * player.a.sin();

    let mut blocked = false;

    let probe_x = player.pos.x + dx + BODY_RADIUS * dx.signum();
    if free_at(maze, block_size, probe_x, player.pos.y) {
        player.pos.x += dx;
    } else {
        blocked = true;
    }

    let probe_y = player.pos.y + dy + BODY_RADIUS * dy.signum();
    if free_at(maze, block_size, player.pos.x, probe_y) {
        player.pos.y += dy;
    } else {
        blocked = true;
    }

    blocked
}
