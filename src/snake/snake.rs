use std::iter;

use tiny::math::{Direction, Directionf32, Position, Positionf32};

struct Snake {
    head_position: Positionf32,
    direction: Directionf32,
    length: u32,
}

#[derive(Clone, Copy)]
enum SnakePart {
    Head,
    Body,
}

#[derive(Clone, Copy)]
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

pub struct GameState {
    snake: Snake,
    level_size: (usize, usize),
    level: Vec<Cell>,
}

fn initialize_level(size: (usize, usize)) -> Vec<Cell> {
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

    level
}

impl GameState {
    pub fn new() -> Self {
        let default_size = (60, 20);
        Self {
            snake: Snake {
                head_position: Positionf32 {
                    x: 4.0f32,
                    y: 4.0f32,
                },
                direction: Directionf32 {
                    x: 1.0f32,
                    y: 0.0f32,
                },
                length: 3,
            },
            level_size: default_size,
            level: initialize_level(default_size),
        }
    }

    pub fn update(&mut self) {
        let position = self.snake.head_position;
        let direction = self.snake.direction;
        let new_position = position + direction * 0.1f32;

        println!("{:?} {:?} {:?}", position, direction, new_position);

        self.snake.head_position = new_position;
    }

    pub fn render(&self) {
        // Render the level.
        let mut render = self.level.clone();

        // Render the snake.
        render[self.snake.head_position.x as usize
            + self.level_size.0 * self.snake.head_position.y as usize] =
            Cell::Snake(SnakePart::Head);

        // Print all.
        render
            .chunks(self.level_size.0)
            .map(|line| {
                line.iter()
                    .map(|cell| char::from(*cell))
                    .collect::<String>()
            })
            .for_each(|line| {
                println!("{line}");
            });
    }
}
