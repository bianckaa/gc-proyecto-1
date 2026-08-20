mod audio;
mod caster;
mod framebuffer;
mod maze;
mod minimap;
mod player;
mod screens;
mod text;
mod texture;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::audio::Audio;
use crate::caster::cast_ray;
use crate::framebuffer::{shade, Framebuffer};
use crate::maze::{load_maze, Maze};
use crate::minimap::draw as draw_minimap;
use crate::player::{process_events, MouseLook, Player};
use crate::screens::{draw_level_select, draw_success, draw_welcome, Level};
use crate::text::draw_text;
use crate::texture::{fallback_color, TextureSet};

const BLOCK_SIZE: usize = 64;
const FOV: f32 = PI / 3.0;
const COLUMN_WIDTH: usize = 2;
const WIDTH: usize = 900;
const HEIGHT: usize = 600;

enum Screen {
    Welcome,
    LevelSelect,
    Playing,
    Success,
}

fn draw_sky_and_floor(framebuffer: &mut Framebuffer) {
    let horizon = framebuffer.height / 2;
    for y in 0..horizon {
        let t = y as f32 / horizon as f32;
        let r = (58.0 + 120.0 * t) as u32;
        let g = (62.0 + 78.0 * t) as u32;
        let b = (74.0 + 40.0 * t) as u32;
        let color = (r << 16) | (g << 8) | b;
        for x in 0..framebuffer.width {
            framebuffer.point_color(x, y, color);
        }
    }
    for y in horizon..framebuffer.height {
        let t = (y - horizon) as f32 / horizon as f32;
        let r = (30.0 + 52.0 * t) as u32;
        let g = (24.0 + 36.0 * t) as u32;
        let b = (18.0 + 22.0 * t) as u32;
        let color = (r << 16) | (g << 8) | b;
        for x in 0..framebuffer.width {
            framebuffer.point_color(x, y, color);
        }
    }
}

fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    textures: &TextureSet,
) {
    draw_sky_and_floor(framebuffer);

    let num_columns = framebuffer.width / COLUMN_WIDTH;
    let half_height = framebuffer.height as f32 / 2.0;
    let projection_distance = (framebuffer.width as f32 / 2.0) / (FOV / 2.0).tan();

    for column in 0..num_columns {
        let ratio = column as f32 / num_columns as f32;
        let angle = player.a - FOV / 2.0 + FOV * ratio;
        let intersect = cast_ray(maze, player, angle, BLOCK_SIZE);

        if intersect.impact == ' ' {
            continue;
        }

        let corrected = (intersect.distance * (angle - player.a).cos()).max(1.0);
        let wall_height = (BLOCK_SIZE as f32 / corrected) * projection_distance;
        let wall_top = half_height - wall_height / 2.0;
        let top = wall_top.max(0.0) as usize;
        let bottom = (half_height + wall_height / 2.0).min(framebuffer.height as f32) as usize;
        let brightness = (1.0 - corrected / 1400.0).clamp(0.28, 1.0);

        let texture = textures.for_cell(intersect.impact);
        let base = fallback_color(intersect.impact);
        let x_start = column * COLUMN_WIDTH;

        for y in top..bottom {
            let v = ((y as f32 - wall_top) / wall_height).clamp(0.0, 0.999);
            let sample = if texture.width == 0 {
                base
            } else {
                texture.get_pixel(intersect.tex_x, v)
            };
            let color = shade(sample, brightness);
            for offset in 0..COLUMN_WIDTH {
                framebuffer.point_color(x_start + offset, y, color);
            }
        }
    }
}

fn draw_hud(framebuffer: &mut Framebuffer, fps: u32, level_name: &str) {
    framebuffer.fill_rect(10, 10, 150, 26, 0x14140F);
    framebuffer.rect_border(10, 10, 150, 26, 0x4E8A34);
    let label = format!("FPS: {fps}");
    draw_text(framebuffer, 18, 16, 2, 0xFFE066, &label);

    let name = level_name.to_uppercase();
    let box_width = 16 + name.chars().count() * 12;
    let box_y = framebuffer.height.saturating_sub(38);
    framebuffer.fill_rect(10, box_y, box_width, 26, 0x14140F);
    framebuffer.rect_border(10, box_y, box_width, 26, 0x4E8A34);
    draw_text(framebuffer, 18, box_y + 6, 2, 0xD8E0C0, &name);
}

fn main() {
    let levels = vec![
        Level {
            name: "El Glade".to_string(),
            path: "mazes/glade.txt".to_string(),
        },
        Level {
            name: "Sector Oeste".to_string(),
            path: "mazes/sector_oeste.txt".to_string(),
        },
    ];

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(0x14140F);

    let mut window = match Window::new(
        "Maze Runner - Ray Caster",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ) {
        Ok(window) => window,
        Err(error) => {
            println!("no se pudo crear la ventana: {error}");
            return;
        }
    };

    let textures = TextureSet::load();
    let mut audio = Audio::new();
    let mut mouse = MouseLook::new();

    let frame_delay = Duration::from_millis(66);
    let mut screen = Screen::Welcome;
    let mut selected = 0usize;
    let mut maze: Maze = Vec::new();
    let mut player = Player {
        pos: nalgebra_glm::Vec2::new(0.0, 0.0),
        a: 0.0,
    };
    let mut level_name = String::new();
    let mut enter_was_down = false;
    let mut up_was_down = false;
    let mut down_was_down = false;
    let mut thud_cooldown = 0u32;
    let mut fps = 15u32;
    let mut fps_frames = 0u32;
    let mut fps_timer = Instant::now();
    let mut run_start = Instant::now();
    let mut run_seconds = 0.0f32;

    while window.is_open() {
        let frame_start = Instant::now();

        let enter_down = window.is_key_down(Key::Enter);
        let enter_pressed = enter_down && !enter_was_down;
        enter_was_down = enter_down;

        let up_down = window.is_key_down(Key::Up);
        let up_pressed = up_down && !up_was_down;
        up_was_down = up_down;

        let down_down = window.is_key_down(Key::Down);
        let down_pressed = down_down && !down_was_down;
        down_was_down = down_down;

        let escape = window.is_key_down(Key::Escape);

        framebuffer.clear();

        match screen {
            Screen::Welcome => {
                if escape {
                    break;
                }
                audio.stop_music();
                draw_welcome(&mut framebuffer);
                if enter_pressed {
                    screen = Screen::LevelSelect;
                }
            }
            Screen::LevelSelect => {
                if escape {
                    screen = Screen::Welcome;
                }
                if up_pressed && selected > 0 {
                    selected -= 1;
                }
                if down_pressed && selected + 1 < levels.len() {
                    selected += 1;
                }
                draw_level_select(&mut framebuffer, &levels, selected);
                if enter_pressed {
                    let level = &levels[selected];
                    let loaded = load_maze(&level.path, BLOCK_SIZE);
                    maze = loaded.0;
                    player = loaded.1;
                    level_name = level.name.clone();
                    mouse.reset();
                    thud_cooldown = 0;
                    run_start = Instant::now();
                    audio.start_music();
                    screen = Screen::Playing;
                }
            }
            Screen::Playing => {
                if escape {
                    audio.stop_music();
                    screen = Screen::Welcome;
                }

                let blocked = process_events(&window, &mut player, &maze, BLOCK_SIZE, &mut mouse);

                if thud_cooldown > 0 {
                    thud_cooldown -= 1;
                }
                if blocked && thud_cooldown == 0 {
                    audio.play_thud();
                    thud_cooldown = 6;
                }

                render_world(&mut framebuffer, &maze, &player, &textures);
                draw_minimap(&mut framebuffer, &maze, &player, BLOCK_SIZE);
                draw_hud(&mut framebuffer, fps, &level_name);

                let i = player.pos.x as usize / BLOCK_SIZE;
                let j = player.pos.y as usize / BLOCK_SIZE;
                if maze.get(j).and_then(|row| row.get(i)) == Some(&'g') {
                    run_seconds = run_start.elapsed().as_secs_f32();
                    audio.stop_music();
                    audio.play_victory();
                    screen = Screen::Success;
                }
            }
            Screen::Success => {
                if escape {
                    break;
                }
                draw_success(&mut framebuffer, &level_name, run_seconds);
                if enter_pressed {
                    screen = Screen::Welcome;
                }
            }
        }

        if window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .is_err()
        {
            break;
        }

        fps_frames += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps = fps_frames;
            fps_frames = 0;
            fps_timer = Instant::now();
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_delay {
            std::thread::sleep(frame_delay - elapsed);
        }
    }
}
