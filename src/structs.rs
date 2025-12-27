#[derive(Copy, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}
pub struct Cell {
    pub n: bool,
    pub s: bool,
    pub w: bool,
    pub e: bool,
}

pub enum Wall {
    North,
    South,
    West,
    East,
}
impl Pos {
    pub fn get_by_indx(pos: Pos,maze: Vec<Vec<bool>>) -> Cell{
        let mut north: bool;
        let mut south: bool;
        let mut west: bool;
        let mut east: bool;
        if pos.y == 0 {
            north = maze[pos.y][pos.x]; //north1
            south = maze[pos.y + 2][pos.x]; //north1
            if pos.x == 0 {
                west = maze[pos.y][pos.x+1];
                east =  maze[pos.y][pos.x+2];
            }//west1
            else {
                west = maze[pos.y+1][pos.x+1];
                east = maze[pos.y+1][pos.x+2];
            }//west2
        }
        else {
            north = maze[pos.y+1][pos.x];//north1
            south = maze[pos.y+3][pos.x]; //north1
            west = maze[pos.y+2][pos.x+1];
            east = maze[pos.y+2][pos.x+2];
        }
        let cell: Cell = Cell { n: (north), s: (south), w: (west), e: (east) };
        cell
    }
    pub fn remove(pos:Pos,  maze: &mut Vec<Vec<bool>>, wall: Wall) -> &mut Vec<Vec<bool>> {
        if pos.y == 0 {
            match wall {
                Wall::North => maze[pos.y][pos.x] = false,
                Wall::South => maze[pos.y + 2][pos.x] = false,
                _ => {}
            }
            if pos.x == 0 {
                match wall {
                    Wall::West => maze[pos.y][pos.x+1] = false,
                    Wall::East => maze[pos.y][pos.x+2] = false,
                    _ => {}
                }
            }
            else {
                match wall {
                    Wall::West => maze[pos.y+1][pos.x+1] = false,
                    Wall::East  =>maze[pos.y+1][pos.x+2] = false,
                    _ => {}
                }
            }//west2
        }
        else {
            match wall {
                Wall::North => maze[pos.y+1][pos.x] = false,
                Wall::South => maze[pos.y+3][pos.x] = false,
                Wall::West => maze[pos.y+2][pos.x+1] = false,
                Wall::East => maze[pos.y+2][pos.x+2] = false,   
            }
        }
        maze
    }
}
