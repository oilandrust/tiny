use std::{collections::HashMap, ops::Add};

use platform::Platform;

mod platform;

const LEVEL_1: &'static str = "######
#.@..#
#X...#
#..Q.#
#....#
######";

const LEVEL_2: &'static str = "#######
#.@...#
#XX...#
#..QQ.#
#.....#
#######";

const LEVELS: [&'static str; 2] = [LEVEL_1, LEVEL_2];

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Player,
    Load(i32),
    Target,
}

impl From<Cell> for char {
    fn from(cell: Cell) -> char {
        match cell {
            Cell::Wall => '#',
            Cell::Empty => ' ',
            Cell::Player => '@',
            Cell::Load(_) => 'Q',
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
            'Q' => Ok(Cell::Load(0)),
            'X' => Ok(Cell::Target),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Level {
    grid: Grid,
    start_position: Position,
    load_positions: HashMap<i32, Position>,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Cell>,
    width: usize,
}

impl Grid {
    fn cell_at(&self, position: &Position) -> Cell {
        self.grid[position.0 as usize + self.width * position.1 as usize]
    }

    fn set_cell(&mut self, position: &Position, value: Cell) {
        self.grid[position.0 as usize + self.width * position.1 as usize] = value;
    }

    fn is_empty(&self, position: &Position) -> bool {
        let cell = self.cell_at(position);
        cell == Cell::Empty || cell == Cell::Target
    }

    fn player_can_move(&self, from_position: &Position, direction: &Direction) -> bool {
        let to_position = *from_position + *direction;
        if self.is_empty(&to_position) {
            return true;
        }

        let next_position = to_position + *direction;
        match self.cell_at(&to_position) {
            Cell::Load(_) => self.is_empty(&next_position),
            _ => false,
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

    // Find the player start position.
    let start_index = grid
        .iter()
        .position(|&cell| cell == Cell::Player)
        .ok_or("Level is missing a player position.".to_string())?;

    let start_position = Position((start_index % width) as i32, (start_index / width) as i32);

    // Find the indices and positions of the loads.
    let load_indices: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|&(_, &cell)| {
            if let Cell::Load(_) = cell {
                true
            } else {
                false
            }
        })
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
        grid: Grid { grid, width },
        start_position,
        load_positions: load_hashmap,
    })
}

fn print_grid(grid: &Grid) {
    for line in grid.grid.chunks(grid.width as usize) {
        let line_string = line
            .iter()
            .map(|cell| <Cell as Into<char>>::into(*cell))
            .collect::<String>();

        println!("{line_string}");
    }
}

#[derive(Debug, Clone, Copy)]
struct Position(i32, i32);

#[derive(Debug, Clone, Copy)]
struct Direction(i32, i32);

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, other: Direction) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

enum Command {
    Move(i32, i32),
    RestartLevel,
    Quit,
}

fn translate_input(input: char) -> Option<Command> {
    match input {
        'w' => Some(Command::Move(0, -1)),
        's' => Some(Command::Move(0, 1)),
        'd' => Some(Command::Move(1, 0)),
        'a' => Some(Command::Move(-1, 0)),
        'r' => Some(Command::RestartLevel),
        'q' => Some(Command::Quit),
        _ => None,
    }
}

struct GameState {
    player_position: Position,
    load_positions: HashMap<i32, Position>,
    level: Level,
}

impl GameState {
    fn load_level(level_string: &str) -> Result<GameState, String> {
        let Ok(level) = parse_level(level_string) else {
            return Err("Failed parsing level.".to_string());
        };

        Ok(GameState {
            player_position: level.start_position,
            load_positions: level.load_positions.clone(),
            level,
        })
    }

    fn render_grid(&self) -> Grid {
        let mut new_grid = self.level.grid.clone();

        new_grid.set_cell(&self.player_position, Cell::Player);
        for (load_id, position) in &self.load_positions {
            new_grid.set_cell(position, Cell::Load(*load_id));
        }

        new_grid
    }

    fn reset(&mut self) {
        self.player_position = self.level.start_position;
        self.load_positions = self.level.load_positions.clone();
    }

    fn move_player(&mut self, grid: &Grid, direction: &Direction) {
        assert!(grid.player_can_move(&self.player_position, &direction));

        let to_position = self.player_position + *direction;

        // Move the load if there is one and it can move.
        if let Cell::Load(uid) = grid.cell_at(&to_position) {
            if let Some(load_position) = self.load_positions.get_mut(&uid) {
                *load_position = to_position + *direction;
            } else {
                panic!("Got an id of an unnexisting load.");
            }
        }

        self.player_position = to_position;
    }

    fn level_is_complete(&self) -> bool {
        for load_position in self.load_positions.values() {
            if self.level.grid.cell_at(load_position) != Cell::Target {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut platform = Platform::new();

    let mut level_iter = LEVELS.iter();
    let Ok(mut game_state) = GameState::load_level(level_iter.next().unwrap()) else {
        return;
    };

    loop {
        let current_grid = game_state.render_grid();

        Platform::clear_display();
        print_grid(&current_grid);

        let input_char = platform.read_char();

        // TODO: change into while to avoid redrawing when input isn't a command.
        let Some(command) = translate_input(input_char) else {
            continue;
        };

        match command {
            Command::Move(dx, dy) => {
                let direction = Direction(dx, dy);
                if current_grid.player_can_move(&game_state.player_position, &direction) {
                    game_state.move_player(&current_grid, &direction);
                }
            }
            Command::RestartLevel => {
                game_state.reset();
            }
            Command::Quit => break,
        }

        if !game_state.level_is_complete() {
            continue;
        }

        // Load next level if any.
        match level_iter.next() {
            Some(level_string) => {
                let Ok(new_state) = GameState::load_level(level_string) else {
                    return;
                };
                game_state = new_state;
            }
            None => {
                println!("Game complete, congratulations!");
                break;
            }
        }
    }
}
