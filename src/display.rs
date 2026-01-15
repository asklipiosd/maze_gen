use crate::structs::{self, *};
use macroquad::prelude::*;

const SIZE: usize = 20;
fn draw_horisontal_line(start: Pos) { 
    draw_line(start.x as f32, start.y as f32, {start.x + SIZE} as f32, start.y as f32, 2.0, BLACK);
}

fn draw_vertical_line(start: Pos) {
    draw_line(start.x as f32, start.y as f32, start.x as f32, {start.y + SIZE} as f32, 2.0, BLACK);
}

pub async fn display_maze(maze: &[Vec<bool>]) {
    loop {
            clear_background(WHITE);
            display_m(maze).await;
            next_frame().await
        }
}

async fn display_m(maze: &[Vec<bool>]) {
    for (y, row) in maze.iter().enumerate() {
        let is_horizontal_row = y % 2 != 0;
        for (x, cell) in row.iter().enumerate() {
            if is_horizontal_row {
                draw_horisontal_line(structs::Pos { x: (x + SIZE), y});
            } 
            else if *cell {
                draw_vertical_line(structs::Pos {x , y: (y + SIZE)});
            }
        }
    }
}
