use crate::structs::*;
use macroquad::prelude::*;

fn draw_horisontal_line(start: Pos) {
    draw_line(start.x as f32, start.y as f32, {start.x + 20} as f32, start.y as f32, 2.0, BLACK);
}

fn draw_vertical_line(start: Pos) {
    draw_line(start.x as f32, start.y as f32, start.x as f32, {start.y + 20} as f32, 2.0, BLACK);
}

pub async fn display_maze(maze: &Vec<Vec<bool>>) {
    loop {
            clear_background(WHITE);
            display_m(maze).await;
            next_frame().await
        }
}

async fn display_m(maze: &Vec<Vec<bool>>) {
    for i in 0..maze.len() {
        if i % 2 != 0 {
            for y in 0..maze[i].len() {
                if maze[i][y] {
                    draw_horisontal_line(Pos { x: (y * 20), y: (i * 20) });
                }
            }
        }
        else {
            for y in 0..maze[i].len() {
                if maze[i][y] {
                    draw_vertical_line(Pos { x: (y * 20), y: (i * 20) });
                }
            }
        }
    }
}