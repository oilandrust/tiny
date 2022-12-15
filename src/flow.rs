use crate::sokoban::*;

pub trait Flow {
    fn render(&self) {}

    fn update(&mut self, _command: Command) -> Option<Box<dyn Flow>> {
        None
    }

    fn should_quit(&self) -> bool {
        false
    }
}

pub struct AppFlow {
    flow: Box<dyn Flow>,
}

impl AppFlow {
    pub fn new() -> Self {
        AppFlow {
            flow: Box::new(IntroFlow {}),
        }
    }

    pub fn render(&self) {
        self.flow.render();
    }

    pub fn update(&mut self, command: Command) {
        if let Some(new_flow) = self.flow.update(command) {
            self.flow = new_flow;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.flow.should_quit()
    }
}

struct IntroFlow {}
struct LevelIntroFlow {
    level_index: usize,
}
pub struct GameFlow {
    current_grid: Grid,
    game_state: GameState,
    current_level_index: usize,
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
            GameFlow::new().expect("Failed to initialize game!"),
        ))
    }
}

impl Flow for LevelIntroFlow {
    fn render(&self) {
        print!(
            "{}",
            LEVEL_INTRO.replace('x', &self.level_index.to_string())
        );
    }

    fn update(&mut self, command: Command) -> Option<Box<dyn Flow>> {
        if command == Command::Quit {
            return Some(Box::new(QuitFlow {}));
        }

        let Ok(new_state) = GameState::load_level(LEVELS[self.level_index]) else {
            panic!("Invalid level");
        };

        let new_grid = new_state.render_grid();

        Some(Box::new(GameFlow {
            current_grid: new_grid,
            game_state: new_state,
            current_level_index: self.level_index,
        }))
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
    fn new() -> Result<Self, String> {
        let mut level_iter = LEVELS.iter();
        let game_state = GameState::load_level(level_iter.next().unwrap())?;
        let initial_grid = game_state.render_grid();

        Ok(GameFlow {
            game_state,
            current_grid: initial_grid,
            current_level_index: 0,
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
                    .player_can_move(&self.game_state.player_position, &direction)
                {
                    self.game_state.move_player(&self.current_grid, &direction);
                }
            }
            Command::RestartLevel => {
                self.game_state.reset();
            }
            Command::Quit => {
                return Some(Box::new(QuitFlow {}));
            }
            _ => {}
        }

        self.current_grid = self.game_state.render_grid();
        if !self.game_state.level_is_complete() {
            return None;
        }

        // Load next level if any.
        self.current_level_index += 1;
        if self.current_level_index < LEVELS.len() {
            Some(Box::new(LevelIntroFlow {
                level_index: self.current_level_index,
            }))
        } else {
            Some(Box::new(EndFlow {}))
        }
    }
}
