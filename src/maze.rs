use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};

use nalgebra_glm::Vec2;

use crate::player::Player;

pub type Maze = Vec<Vec<char>>;

pub fn is_walkable(cell: char) -> bool {
    cell == ' ' || cell == 'g'
}

pub fn cell_at(maze: &Maze, block_size: usize, x: f32, y: f32) -> char {
    if x < 0.0 || y < 0.0 {
        return '+';
    }
    let i = x as usize / block_size;
    let j = y as usize / block_size;
    match maze.get(j).and_then(|row| row.get(i)) {
        Some(&cell) => cell,
        None => '+',
    }
}

fn fallback_maze() -> Maze {
    let rows = [
        "++++++++++++",
        "+          +",
        "+ ++++ +++ +",
        "+      +   +",
        "+ ++++ + + +",
        "+    + + + +",
        "++++ + + + +",
        "+        +g+",
        "++++++++++++",
    ];
    rows.iter().map(|row| row.chars().collect()).collect()
}

pub fn load_maze(filename: &str, block_size: usize) -> (Maze, Player) {
    let mut maze: Maze = Vec::new();
    let mut player_pos: Option<Vec2> = None;

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (row, line) in reader.lines().enumerate() {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => continue,
                };
                let mut cells: Vec<char> = Vec::new();
                for (col, character) in line.chars().enumerate() {
                    if character == 'p' {
                        let x = col * block_size + block_size / 2;
                        let y = row * block_size + block_size / 2;
                        player_pos = Some(Vec2::new(x as f32, y as f32));
                        cells.push(' ');
                    } else {
                        cells.push(character);
                    }
                }
                if !cells.is_empty() {
                    maze.push(cells);
                }
            }
        }
        Err(_) => {
            println!("no se pudo abrir {filename}, se usa el laberinto de respaldo");
        }
    }

    if maze.is_empty() {
        maze = fallback_maze();
        player_pos = None;
    }

    let start = player_pos.unwrap_or_else(|| {
        let mut found = Vec2::new(block_size as f32 * 1.5, block_size as f32 * 1.5);
        for (row, line) in maze.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if cell == ' ' {
                    found = Vec2::new(
                        (col * block_size + block_size / 2) as f32,
                        (row * block_size + block_size / 2) as f32,
                    );
                    return found;
                }
            }
        }
        found
    });

    let player = Player {
        pos: start,
        a: PI / 4.0,
    };

    (maze, player)
}
