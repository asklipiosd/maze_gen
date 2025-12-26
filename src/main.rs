mod structs;
mod dfs;

use dfs::*;
use structs::*;
use macroquad::prelude::*;

pub fn draw_simple_line(start: Pos, end: Pos) {
    draw_line(start.x as f32, start.y as f32, end.x as f32, end.y as f32, 2.0, BLACK);
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let rows = 5;
    let x = 5;
    let mut visited_cells = vec![vec![true; x]; rows];
    let mut maze: Vec<Vec<bool>> = (0..rows)
        .map(|row| {
            let cols = if row % 2 == 1 { 2*x } else { x };
            vec![true; cols]
        })
    .collect();
    loop {
        clear_background(WHITE);
        
        draw_line(40.0, 40.0, 80.0, 40.0, 2.0, BLACK);
        next_frame().await
    }
}
