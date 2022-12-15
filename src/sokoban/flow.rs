use crate::sokoban::*;
use tinylib::flow::{Command, Flow};

pub struct IntroFlow {}

pub struct GameFlow {
    current_grid: Grid,
    game_state: GameState,
    level_index: usize,
}

struct EndFlow {}

struct QuitFlow {}

impl Flow for IntroFlow {
    fn render(&self) {
        print!("{INTRO}");
    }

    fn update(&mut self, command: Command) -> Option<Box<dyn Flow>> {
        if command == Command::Quit {
            return Some(Box::new(QuitFlow {}));
        }

        Some(Box::new(
            GameFlow::new(0).expect("Failed to initialize game!"),
        ))
    }
}

impl Flow for EndFlow {
    fn render(&self) {
        print!("{END}");
    }

    fn update(&mut self, _command: Command) -> Option<Box<dyn Flow>> {
        Some(Box::new(QuitFlow {}))
    }
}

impl Flow for QuitFlow {
    fn should_quit(&self) -> bool {
        true
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

    fn update(&mut self, command: Command) -> Option<Box<dyn Flow>> {
        match command {
            Command::Move(dx, dy) => {
                let direction = Direction(dx, dy);
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
