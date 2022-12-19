use std::{collections::VecDeque, iter};

use rand::Rng;
use tiny::prelude::*;

#[derive(Clone)]
pub struct Snake {
    direction: Direction,
    pub parts: VecDeque<Position>,
    accumulated_distance: f32,
}

enum AdvanceResult {
    Collision,
    Ok(Position),
}

impl Snake {
    fn new(head_position: Position, length: usize) -> Self {
        let Position { x, y } = head_position;
        let mut parts = VecDeque::from([head_position]);
        for i in 0..length {
            parts.push_back(Position {
                x: x - (i + 1) as i32,
                y,
            });
        }

        Snake {
            direction: Direction { x: 1, y: 0 },
            parts,
            accumulated_distance: 0.0f32,
        }
    }

    fn head_position(&self) -> Position {
        assert!(!self.parts.is_empty());

        *self.parts.front().unwrap()
    }

    fn advance(&mut self, grid: &Grid, delta_time: Duration) -> AdvanceResult {
        assert!(!self.parts.is_empty());

        let speed = 1.0f32 * self.parts.len() as f32;
        self.accumulated_distance += speed * delta_time.as_secs_f32();
        if self.accumulated_distance > 1.0f32 {
            let new_head_position = *self.parts.front().unwrap() + self.direction;

            if !grid.is_empty(new_head_position) {
                return AdvanceResult::Collision;
            }

            for part in &self.parts {
                if *part == new_head_position {
                    return AdvanceResult::Collision;
                }
            }

            self.parts.push_front(new_head_position);
            self.parts.pop_back();
            self.accumulated_distance = 0.0f32;
        }

        return AdvanceResult::Ok(*self.parts.back().unwrap());
    }

    fn grow(&mut self, tail_position: Position) {
        self.parts.push_back(tail_position);
    }

    fn set_direction(&mut self, new_direction: Direction) {
        if new_direction == self.direction || -new_direction == self.direction {
            return;
        }
        self.direction = new_direction;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnakePart {
    Head,
    Body,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Wall,
    Food,
    Snake(SnakePart),
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Empty => ' ',
            Cell::Wall => '#',
            Cell::Food => 'Q',
            Cell::Snake(SnakePart::Head) => '@',
            Cell::Snake(SnakePart::Body) => 'a',
        }
    }
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
        cell == Cell::Empty || cell == Cell::Food
    }

    pub fn print(&self) {
        for line in self.grid.chunks(self.width as usize) {
            let line_string = line
                .iter()
                .map(|cell| char::from(*cell))
                .collect::<String>();

            println!("{line_string}");
        }
    }
}

pub struct GameState {
    pub snake: Snake,
    pub grid: Grid,
    food_spawmer: FoodSpawner,
    pub foods: Vec<Position>,
}

fn initialize_level(size: (usize, usize)) -> Grid {
    let interior_size = (size.0 - 2, size.1 - 2);

    let mut level = Vec::with_capacity((size.0 * size.1) as usize);
    level.extend(iter::repeat(Cell::Wall).take(size.0 as usize));
    for _ in 0..interior_size.1 {
        level.extend(
            iter::once(Cell::Wall).chain(
                iter::repeat(Cell::Empty)
                    .take(interior_size.0 as usize)
                    .chain(iter::once(Cell::Wall)),
            ),
        );
    }
    level.extend(iter::repeat(Cell::Wall).take(size.0 as usize));

    Grid {
        grid: level,
        width: size.0,
        height: size.1,
    }
}

struct FoodSpawner {
    time_to_next_spawn: Duration,
    time_since_last_spawn: Duration,
}

impl FoodSpawner {
    fn update_and_spawn(&mut self, grid: &Grid, delta_time: Duration) -> Option<Position> {
        self.time_since_last_spawn += delta_time;
        if self.time_since_last_spawn > self.time_to_next_spawn {
            self.time_since_last_spawn = Duration::new(0, 0);

            let mut rng = rand::thread_rng();
            self.time_to_next_spawn = Duration::new(rng.gen_range(5..10), 0);

            return Some(Position {
                x: rng.gen_range(1..grid.width - 1) as i32,
                y: rng.gen_range(1..grid.height - 1) as i32,
            });
        }

        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdateResult {
    Collision,
    Ok,
}

impl GameState {
    pub fn new() -> Self {
        let default_size = (60, 30);

        Self {
            snake: Snake::new(Position { x: 3, y: 1 }, 3),
            grid: initialize_level(default_size),
            food_spawmer: FoodSpawner {
                time_to_next_spawn: Duration::new(0, 0),
                time_since_last_spawn: Duration::new(0, 0),
            },
            foods: vec![],
        }
    }

    pub fn update(&mut self, delta_time: Duration) -> UpdateResult {
        match self.snake.advance(&self.grid, delta_time) {
            AdvanceResult::Collision => {
                return UpdateResult::Collision;
            }
            AdvanceResult::Ok(tail_position) => {
                let head_position = self.snake.head_position();
                for food_position in &self.foods {
                    if *food_position == head_position {
                        self.snake.grow(tail_position);
                    }
                }

                self.foods = self
                    .foods
                    .clone()
                    .into_iter()
                    .filter(|position| *position != head_position)
                    .collect();
            }
        };

        if let Some(spawn_position) = self.food_spawmer.update_and_spawn(&self.grid, delta_time) {
            self.foods.push(spawn_position);
        }

        UpdateResult::Ok
    }

    pub fn render(&self) {
        // Render the level.
        let mut render = self.grid.clone();

        // Render the snake.
        for part in self.snake.parts.iter() {
            render.set_cell(*part, Cell::Snake(SnakePart::Body));
        }
        render.set_cell(
            *self.snake.parts.front().unwrap(),
            Cell::Snake(SnakePart::Head),
        );

        for food in &self.foods {
            render.set_cell(*food, Cell::Food);
        }

        // Print all.
        render.print();
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.snake.set_direction(new_direction);
    }
}
