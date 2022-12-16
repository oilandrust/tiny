use tinylib::{
    flow::Flow,
    flows::{GameLauncher, QuitFlow},
    platform::Key,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Move(i32, i32),
    Quit,
    Unknown,
}

pub fn translate_input(input: Key) -> Command {
    match input {
        Key::W => Command::Move(0, -1),
        Key::S => Command::Move(0, 1),
        Key::D => Command::Move(1, 0),
        Key::A => Command::Move(-1, 0),
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
        Box::new(QuitFlow)
    }
}

pub struct GameFlow {}

impl GameFlow {
    fn new() -> Result<Self, String> {
        Ok(GameFlow {})
    }
}

impl Flow for GameFlow {
    fn render(&self) {}

    fn update(&mut self, key: Key) -> Option<Box<dyn Flow>> {
        let command = translate_input(key);
        match command {
            Command::Move(dx, dy) => {}
            Command::Quit => {
                return Some(Box::new(QuitFlow {}));
            }
            _ => {}
        }

        None
    }
}
