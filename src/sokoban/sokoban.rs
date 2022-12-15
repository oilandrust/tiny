use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use tinylib::flow::Command;

pub const INTRO: &str = "#########################
#                       #
#     Sokoban Mini      #
#                       #
#                       #
#     wasd -> move      #
#       r  -> reset     #
#       u  -> undo      #
#       q  -> quit      #
#                       #
#   Any key to start!   #
#                       #
#########################";

pub const END: &str = "#########################
#                       #
#       The End!        #
# All levels completed. #
#                       #
#   Congratulations!    #
#                       #
#   Any key to quit...  #
#                       #
#########################";

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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
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
    grid: Grid,
    start_position: Position,
    box_positions: HashMap<i32, Position>,
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn cell_at(&self, position: Position) -> Cell {
        self.grid[position.0 as usize + self.width * position.1 as usize]
    }

    fn set_cell(&mut self, position: Position, value: Cell) {
        self.grid[position.0 as usize + self.width * position.1 as usize] = value;
    }

    fn is_empty(&self, position: Position) -> bool {
        let cell = self.cell_at(position);
        cell == Cell::Empty || cell == Cell::Target
    }

    pub fn player_can_move(&self, from_position: Position, direction: Direction) -> bool {
        let to_position = from_position + direction;

        let Position(x, y) = to_position;
        if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
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

fn parse_level(level_string: &str) -> Result<Level, String> {
    let width = level_string
        .chars()
        .position(|char| char == '\n')
        .ok_or("Cound not find level width")?;

    let mut grid: Vec<Cell> = level_string
        .chars()
        .filter(|char| *char != '\n')
        .filter_map(|char| char.try_into().ok())
        .collect();

    let height = grid.len() / width;

    // Find the player start position.
    let start_index = grid
        .iter()
        .position(|&cell| cell == Cell::Player)
        .ok_or_else(|| "Level is missing a player position.".to_string())?;

    let start_position = Position((start_index % width) as i32, (start_index / width) as i32);

    // Find the indices and positions of the loads.
    let load_indices: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_, &cell)| matches!(cell, Cell::Box(_)))
        .map(|(index, _)| index)
        .collect();
    let load_positions: Vec<Position> = load_indices
        .iter()
        .map(|index| Position((index % width) as i32, (index / width) as i32))
        .collect();

    // Set the cells where the player and loads are as empty, they are manages as part of the game state.
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

#[derive(Debug, Clone, Copy)]
pub struct Position(i32, i32);

#[derive(Debug, Clone, Copy)]
pub struct Direction(pub i32, pub i32);

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, other: Direction) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Direction> for Position {
    type Output = Self;

    fn sub(self, other: Direction) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

pub fn translate_input(input: char) -> Command {
    match input {
        'w' => Command::Move(0, -1),
        's' => Command::Move(0, 1),
        'd' => Command::Move(1, 0),
        'a' => Command::Move(-1, 0),
        'r' => Command::RestartLevel,
        'q' => Command::Quit,
        'u' => Command::Undo,
        _ => Command::Unknown,
    }
}

struct Move {
    player_move: Direction,
    box_move: Option<i32>,
}

pub struct GameState {
    pub player_position: Position,
    box_positions: HashMap<i32, Position>,
    level: Level,
    move_history: Vec<Move>,
}

impl GameState {
    fn new(level: Level) -> Self {
        GameState {
            player_position: level.start_position,
            box_positions: level.box_positions.clone(),
            level,
            move_history: vec![],
        }
    }

    pub fn load_level(level_string: &str) -> Result<Self, String> {
        let Ok(level) = parse_level(level_string) else {
            return Err("Failed parsing level.".to_string());
        };

        Ok(GameState::new(level))
    }

    pub fn render_grid(&self) -> Grid {
        let mut new_grid = self.level.grid.clone();

        new_grid.set_cell(self.player_position, Cell::Player);
        for (load_id, position) in &self.box_positions {
            new_grid.set_cell(*position, Cell::Box(*load_id));
        }

        new_grid
    }

    pub fn reset(&mut self) {
        let level = self.level.clone();
        *self = GameState::new(level);
    }

    pub fn undo(&mut self) {
        let Some(move_item) = self.move_history.pop() else {
            return;
        };

        self.player_position = self.player_position - move_item.player_move;

        if let Some(box_id) = move_item.box_move {
            if let Some(load_position) = self.box_positions.get_mut(&box_id) {
                let old_position = *load_position - move_item.player_move;
                *load_position = old_position;
            } else {
                panic!("Got an id of an unnexisting box.");
            }
        }
    }

    pub fn move_player(&mut self, grid: &Grid, direction: Direction) {
        assert!(grid.player_can_move(self.player_position, direction));

        let to_position = self.player_position + direction;

        let mut move_item = Move {
            player_move: direction,
            box_move: None,
        };

        // Move the load if there is one and it can move.
        if let Cell::Box(uid) = grid.cell_at(to_position) {
            if let Some(load_position) = self.box_positions.get_mut(&uid) {
                *load_position = to_position + direction;
                move_item.box_move = Some(uid);
            } else {
                panic!("Got an id of an unnexisting load.");
            }
        }

        self.player_position = to_position;
        self.move_history.push(move_item);
    }

    pub fn level_is_complete(&self) -> bool {
        for load_position in self.box_positions.values() {
            if self.level.grid.cell_at(*load_position) != Cell::Target {
                return false;
            }
        }

        true
    }
}
