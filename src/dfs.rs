use crate::structs::{self, *};
use ::rand::random_range;
use macroquad::prelude::*;

fn random_direction() -> Wall {
    match random_range(0..4) {
        0 => Wall::North,
        1 => Wall::South,
        2 => Wall::East,
        _ => Wall::West,
    }
}

pub fn recursive_backtracker(mut pos: Pos,mut maze: &mut Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>) {
    let all_true = visited.iter().all(|row| row.iter().all(|&cell| cell));
    if all_true {
        return;
    }

    else if visited.get(pos.y).and_then(|row| row.get(pos.x+1)).copied().unwrap_or(false)
        || visited.get(pos.y).and_then(|row| row.get(pos.x-1)).copied().unwrap_or(false)
        || visited.get(pos.y + 1).and_then(|row| row.get(pos.x)).copied().unwrap_or(false)
        || visited.get(pos.y - 1).and_then(|row| row.get(pos.x)).copied().unwrap_or(false)
    {}
    else {
        visited[pos.y][pos.x] = true;
        loop {
            match ::rand::random_range(0..4) {
                0 => {
                    if visited.get(pos.y).and_then(|row: &Vec<bool>| row.get(pos.x+1)).copied().unwrap_or(false) {continue;}
                    else {
                        maze = structs::Pos::remove(pos, maze, Wall::North);
                        pos = Pos { x: (pos.x), y: (pos.y + 1) };
                        break;
                    }
                },
                1 => {
                    if visited.get(pos.y).and_then(|row: &Vec<bool>| row.get(pos.x-1)).copied().unwrap_or(false) {continue;}
                    else{
                        pos = Pos { x: (pos.x), y: (pos.y - 1) };
                        maze = structs::Pos::remove(pos, maze, Wall::South);
                        break;
                    } 
                },
                2 => {
                    if visited.get(pos.y + 1).and_then(|row: &Vec<bool>| row.get(pos.x)).copied().unwrap_or(false) {continue;}
                    else {
                        pos = Pos { x: (pos.x + 1), y: (pos.y) };
                        maze = structs::Pos::remove(pos, maze, Wall::East);
                        break;
                    }
                },
                _ => {
                    if visited.get(pos.y-1).and_then(|row: &Vec<bool>| row.get(pos.x)).copied().unwrap_or(false) {continue;}
                    else {
                        pos = Pos { x: (pos.x - 1), y: (pos.y) };
                        maze = structs::Pos::remove(pos, maze, Wall::West);
                    break;
                    }
                },
            }
        }
        recursive_backtracker(pos, maze, visited)
    }
}
