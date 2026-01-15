use crate::structs::{self, *};
use macroquad::prelude::*;

fn has_visited(pos: structs::Pos, visited: &mut [Vec<bool>]) -> bool {
    (pos.x + 1 < visited[0].len() && visited[pos.y][pos.x + 1])
    || (pos.x > 0 && visited[pos.y][pos.x - 1])
    || (pos.y + 1 < visited.len() && visited[pos.y + 1][pos.x])
    || (pos.y > 0 && visited[pos.y - 1][pos.x])
}
pub fn recursive_backtracker(mut pos: Pos, maze: &mut Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>) {
    let mut has_visited_neighbor = has_visited(pos, visited);
    
    let cell = structs::Pos::get_by_indx(pos, maze);
    visited[pos.y][pos.x] = true;
    while !has_visited_neighbor {
        let choice = structs::Wall::random_wall();
        if 
            (choice == structs::Wall::North && pos.x + 1 < visited[0].len() && visited[pos.y][pos.x + 1])
            || (choice == structs::Wall::South && pos.x > 0 && visited[pos.y][pos.x - 1])
            || (choice == structs::Wall::East && pos.y + 1 < visited.len() && visited[pos.y + 1][pos.x])
            || (choice == structs::Wall::West && pos.y > 0 && visited[pos.y - 1][pos.x])
        {continue;}
        else {
            match choice {
                structs::Wall::North => {
                    pos = structs::Pos{x:pos.x ,y:pos.y + 1};
                    recursive_backtracker(pos, maze,visited);
                    structs::Pos::remove(pos, maze, structs::Wall::North);
                    has_visited_neighbor = has_visited(pos, visited);
                }
                structs::Wall::South => {
                    pos = structs::Pos{x:pos.x ,y:pos.y - 1};
                    recursive_backtracker(pos, maze,visited);
                    structs::Pos::remove(pos, maze, structs::Wall::South);
                    has_visited_neighbor = has_visited(pos, visited);

                }
                structs::Wall::West => {
                    pos = structs::Pos{x:pos.x + 1 ,y:pos.y};
                    recursive_backtracker(pos, maze,visited);
                    structs::Pos::remove(pos, maze, structs::Wall::West);
                    has_visited_neighbor = has_visited(pos, visited);
                }
                structs::Wall::East => {
                    pos = structs::Pos{x:pos.x - 1 ,y:pos.y};
                    recursive_backtracker(pos, maze,visited);
                    structs::Pos::remove(pos, maze, structs::Wall::North);
                    has_visited_neighbor = has_visited(pos, visited);
                }
            }
        }
    }
}
