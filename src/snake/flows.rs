use crate::snake::GameState;
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

    fn update(&mut self, delta_time: Duration) -> Option<Box<dyn Flow>> {
        self.state.update(delta_time);
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
