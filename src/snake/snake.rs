use std::{collections::VecDeque, iter};

use tiny::math::{Direction, Position};

struct Snake {
    direction: Direction,
    parts: VecDeque<Position>,
    accumulated_distance: f32,
}

impl Snake {
    fn new(head_position: Position, length: usize) -> Self {
        let Position { x, y } = head_position;
        let parts = VecDeque::from([
            head_position,
            Position { x: x - 1, y },
            Position { x: x - 2, y },
        ]);

        Snake {
            direction: Direction { x: 1, y: 0 },
            parts,
            accumulated_distance: 0.0f32,
        }
    }

    fn advance(&mut self) {
        assert!(!self.parts.is_empty());

        // TODO: Make that framerate independent!
        self.accumulated_distance += 0.2f32;
        if self.accumulated_distance > 1.0f32 {
            let new_head_position = *self.parts.front().unwrap() + self.direction;
            self.parts.push_front(new_head_position);
            self.parts.pop_back();
            self.accumulated_distance = 0.0f32;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SnakePart {
    Head,
    Body,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
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
    snake: Snake,
    grid: Grid,
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

impl GameState {
    pub fn new() -> Self {
        let default_size = (60, 60);

        Self {
            snake: Snake::new(Position { x: 3, y: 1 }, 3),
            grid: initialize_level(default_size),
        }
    }

    pub fn update(&mut self) {
        self.snake.advance();
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

        // Print all.
        render.print();
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        if new_direction == self.snake.direction || -new_direction == self.snake.direction {
            return;
        }

        self.snake.direction = new_direction;
    }
}
