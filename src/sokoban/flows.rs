use std::time::Duration;

use crate::{
    level::{Grid, LEVELS},
    sokoban::*,
};
use tiny::{
    flow::{GameLauncher, QuitFlow},
    prelude::*,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Move(i32, i32),
    RestartLevel,
    Quit,
    Undo,
    Unknown,
}

pub fn translate_input(input: Key) -> Command {
    match input {
        Key::W => Command::Move(0, -1),
        Key::A => Command::Move(-1, 0),
        Key::S => Command::Move(0, 1),
        Key::D => Command::Move(1, 0),
        Key::R => Command::RestartLevel,
        Key::Q => Command::Quit,
        Key::U => Command::Undo,
        _ => Command::Unknown,
    }
}

pub struct SokobanLauncher;

impl GameLauncher for SokobanLauncher {
    fn new() -> Self {
        SokobanLauncher
    }

    fn launch_game(&self) -> Box<dyn Flow> {
        Box::new(GameFlow::new(0).expect("Failed to initialize game!"))
    }
}

pub struct GameFlow {
    current_grid: Grid,
    game_state: GameState,
    level_index: usize,
}

struct EndFlow {}

impl Flow for EndFlow {
    fn render(&self) {
        print!("{END}");
    }

    fn handle_key(&mut self, _key: Key) -> Option<Box<dyn Flow>> {
        Some(Box::new(QuitFlow {}))
    }
}

impl GameFlow {
    fn new(level_index: usize) -> Result<Self, String> {
        let game_state =
            GameState::load_level(LEVELS.get(level_index).ok_or("Invalid level index")?)?;
        let initial_grid = game_state.render_grid();

        Ok(GameFlow {
            game_state,
            current_grid: initial_grid,
            level_index,
        })
    }
}

impl Flow for GameFlow {
    fn render(&self) {
        self.current_grid.print();
    }

    fn handle_key(&mut self, key: Key) -> Option<Box<dyn Flow>> {
        let command = translate_input(key);
        match command {
            Command::Move(dx, dy) => {
                let direction = Direction { x: dx, y: dy };
                if self
                    .current_grid
                    .player_can_move(self.game_state.player_position, direction)
                {
                    self.game_state.move_player(&self.current_grid, direction);
                }
            }
            Command::RestartLevel => {
                self.game_state.reset();
            }
            Command::Quit => {
                return Some(Box::new(QuitFlow {}));
            }
            Command::Undo => {
                self.game_state.undo();
            }
            _ => {}
        }

        None
    }

    fn update(&mut self, _delta_time: Duration) -> Option<Box<dyn Flow>> {
        self.current_grid = self.game_state.render_grid();
        if !self.game_state.level_is_complete() {
            return None;
        }

        // Load next level if any.
        if self.level_index + 1 < LEVELS.len() {
            Some(Box::new(
                GameFlow::new(self.level_index + 1).expect("Failed to load level."),
            ))
        } else {
            Some(Box::new(EndFlow {}))
        }
    }
}
