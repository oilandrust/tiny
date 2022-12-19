use crate::snake::{Cell, GameState, Grid, Snake, SnakePart, UpdateResult};
use tiny::app::Time;
use tiny::prelude::*;

use tiny::flow::{GameLauncher, QuitFlow};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Move(i32, i32),
    Quit,
    Unknown,
}

pub fn translate_input(input: Key) -> Command {
    match input {
        Key::W => Command::Move(0, -1),
        Key::A => Command::Move(-1, 0),
        Key::S => Command::Move(0, 1),
        Key::D => Command::Move(1, 0),
        Key::Q => Command::Quit,
        _ => Command::Unknown,
    }
}

pub struct SnakeLauncher;
impl GameLauncher for SnakeLauncher {
    fn new() -> Self {
        SnakeLauncher
    }

    fn launch_game(&self) -> Box<dyn Flow> {
        Box::new(GameFlow::new())
    }
}

pub struct GameFlow {
    state: GameState,
}

impl GameFlow {
    pub fn new() -> Self {
        GameFlow {
            state: GameState::new(),
        }
    }
}

impl Flow for GameFlow {
    fn render(&self) {
        self.state.render();
    }

    fn update(&mut self, time: &Time) -> Option<Box<dyn Flow>> {
        if self.state.update(time.frame_delta_time) == UpdateResult::Collision {
            return Some(Box::new(CollisionAnimSequence::new(
                &self.state,
                time.time_since_startup,
            )));
        }

        None
    }

    fn handle_key(&mut self, key: Key) -> Option<Box<dyn Flow>> {
        let command = translate_input(key);
        match command {
            Command::Move(dx, dy) => {
                let new_direction = Direction { x: dx, y: dy };
                self.state.set_direction(new_direction);
            }
            Command::Quit => {
                return Some(Box::new(QuitFlow {}));
            }
            _ => {}
        }

        None
    }
}

struct CollisionAnimSequence {
    anim_start_time: Duration,
    snake: Snake,
    grid: Grid,
    foods: Vec<Position>,
    snake_visible: bool,
}

impl CollisionAnimSequence {
    fn new(game_state: &GameState, start_time: Duration) -> Self {
        // TODO: Would like to avoid clone here.
        CollisionAnimSequence {
            anim_start_time: start_time,
            snake: game_state.snake.clone(),
            grid: game_state.grid.clone(),
            foods: game_state.foods.clone(),
            snake_visible: false,
        }
    }
}

impl Flow for CollisionAnimSequence {
    fn update(&mut self, time: &Time) -> Option<Box<dyn Flow>> {
        if time.time_since_startup > self.anim_start_time + Duration::new(3, 0) {
            return Some(Box::new(GameFlow::new()));
        }

        self.snake_visible =
            (3 * (time.time_since_startup - self.anim_start_time)).as_secs() % 2 == 0;

        None
    }

    fn render(&self) {
        // Render the level.
        let mut render = self.grid.clone();

        // Render the snake.
        if self.snake_visible {
            for part in self.snake.parts.iter() {
                render.set_cell(*part, Cell::Snake(SnakePart::Body));
            }
            render.set_cell(
                *self.snake.parts.front().unwrap(),
                Cell::Snake(SnakePart::Head),
            );
        }

        for food in &self.foods {
            render.set_cell(*food, Cell::Food);
        }

        // Print all.
        render.print();
    }
}
