use crate::structs::{self, *};
use rand::prelude::*;

fn random_direction() -> Wall {
    match rand::random_range(0..4) {
        0 => Wall::North,
        1 => Wall::South,
        2 => Wall::East,
        _ => Wall::West,
    }
}

pub fn recursive_backtracker(mut pos: Pos,mut maze: Vec<Vec<bool>>,mut visited: Vec<Vec<bool>>) {
    if visited.get(pos.y).and_then(|row| row.get(pos.x+1)).copied().unwrap_or(true)
        || visited.get(pos.y).and_then(|row| row.get(pos.x-1)).copied().unwrap_or(true)
        || visited.get(pos.y + 1).and_then(|row| row.get(pos.x)).copied().unwrap_or(true)
        || visited.get(pos.y - 1).and_then(|row| row.get(pos.x)).copied().unwrap_or(true)
    {}
    else {
        visited[pos.y][pos.x] = true;
        
        match rand::random_range(0..4) {
            0 => {
                pos = Pos { x: (pos.x), y: (pos.y + 1) };
                structs::Pos::remove(pos, maze.clone(), Wall::North);
            },
            1 => {
                pos = Pos { x: (pos.x), y: (pos.y - 1) };
                structs::Pos::remove(pos, maze.clone(), Wall::South); 
            },
            2 => { 
                pos = Pos { x: (pos.x + 1), y: (pos.y) };
                structs::Pos::remove(pos, maze.clone(), Wall::East);
            },
            _ => {
                pos = Pos { x: (pos.x - 1), y: (pos.y) };
                structs::Pos::remove(pos, maze.clone(), Wall::West);
            },
        }
        recursive_backtracker(pos, maze, visited);
    }
}
