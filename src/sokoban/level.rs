use std::collections::HashMap;

use tiny::math::{Direction, Position};

const LEVEL_0: &str = "####
@.QX
####";

const LEVEL_1: &str = "######
#.@..#
#X...#
#..Q.#
#....#
######";

const LEVEL_2: &str = "#######
#.@...#
#XX...#
#..QQ.#
#.....#
#######";

pub const LEVELS: [&str; 3] = [LEVEL_0, LEVEL_1, LEVEL_2];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Empty,
    Player,
    Box(i32),
    Target,
}

impl From<Cell> for char {
    fn from(cell: Cell) -> char {
        match cell {
            Cell::Wall => '#',
            Cell::Empty => ' ',
            Cell::Player => '@',
            Cell::Box(_) => 'Q',
            Cell::Target => 'X',
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Wall),
            ' ' => Ok(Cell::Empty),
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Player),
            'Q' => Ok(Cell::Box(0)),
            'X' => Ok(Cell::Target),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub grid: Grid,
    pub start_position: Position,
    pub box_positions: HashMap<i32, Position>,
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn cell_at(&self, position: Position) -> Cell {
        self.grid[position.x as usize + self.width * position.y as usize]
    }

    pub fn set_cell(&mut self, position: Position, value: Cell) {
        self.grid[position.x as usize + self.width * position.y as usize] = value;
    }

    pub fn is_empty(&self, position: Position) -> bool {
        let cell = self.cell_at(position);
        cell == Cell::Empty || cell == Cell::Target
    }

    pub fn player_can_move(&self, from_position: Position, direction: Direction) -> bool {
        let to_position = from_position + direction;

        if to_position.x < 0
            || to_position.x > self.width as i32
            || to_position.y < 0
            || to_position.y > self.height as i32
        {
            return false;
        }

        if self.is_empty(to_position) {
            return true;
        }

        let next_position = to_position + direction;
        match self.cell_at(to_position) {
            Cell::Box(_) => self.is_empty(next_position),
            _ => false,
        }
    }

    pub fn print(&self) {
        for line in self.grid.chunks(self.width as usize) {
            let line_string = line
                .iter()
                .map(|cell| <Cell as Into<char>>::into(*cell))
                .collect::<String>();

            println!("{line_string}");
        }
    }
}

pub fn parse_level(level_string: &str) -> Result<Level, String> {
    let mut lines: Vec<Vec<Cell>> = level_string
        .split('\n')
        .map(|line| {
            line.chars()
                .filter_map(|char| char.try_into().ok())
                .collect()
        })
        .collect();

    let width = lines
        .iter()
        .max_by_key(|line| line.len())
        .ok_or("Malformated level")?
        .len();

    let height = lines.len();

    for line in &mut lines {
        line.resize(width, Cell::Empty);
    }

    let mut grid: Vec<Cell> = lines.into_iter().flatten().collect();

    // Find the player start position.
    let start_index = grid
        .iter()
        .position(|&cell| cell == Cell::Player)
        .ok_or_else(|| "Level is missing a player position.".to_string())?;

    let start_position = Position {
        x: (start_index % width) as i32,
        y: (start_index / width) as i32,
    };

    // Find the indices and positions of the loads.
    let load_indices: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_, &cell)| matches!(cell, Cell::Box(_)))
        .map(|(index, _)| index)
        .collect();
    let load_positions: Vec<Position> = load_indices
        .iter()
        .map(|index| Position {
            x: (index % width) as i32,
            y: (index / width) as i32,
        })
        .collect();

    // Set the cells where the player and loads are as empty, they are managed as part of the game state.
    grid[start_index] = Cell::Empty;
    for index in load_indices {
        grid[index] = Cell::Empty;
    }

    // Give an id to the loads.
    let mut load_hashmap = HashMap::new();
    for position in load_positions {
        load_hashmap.insert(load_hashmap.len() as i32, position);
    }

    Ok(Level {
        grid: Grid {
            grid,
            width,
            height,
        },
        start_position,
        box_positions: load_hashmap,
    })
}
