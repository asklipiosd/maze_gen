mod structs;
mod dfs;
mod display;

use display::display_maze;
use structs::*;
use macroquad::prelude::*;

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
    //dfs::recursive_backtracker(Pos { x: (0), y: (0) },&mut maze, &mut visited_cells);
    display_maze(&maze).await;
}
