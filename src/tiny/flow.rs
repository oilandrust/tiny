use std::iter;

use crate::platform::Key;

pub trait Flow {
    fn render(&self) {}

    fn handle_key(&mut self, _key: Key) -> Option<Box<dyn Flow>> {
        None
    }

    fn update(&mut self) -> Option<Box<dyn Flow>> {
        None
    }

    fn should_quit(&self) -> bool {
        false
    }
}

pub struct DefaultFlow;
impl Flow for DefaultFlow {}

pub trait GameLauncher {
    fn new() -> Self;

    fn launch_game(&self) -> Box<dyn Flow>;
}

#[derive(Default)]
pub struct IntroFlow<Launcher: GameLauncher> {
    app_name: String,
    controls: Vec<(String, String)>,
    launcher: Launcher,
}

impl<Launcher: GameLauncher> IntroFlow<Launcher> {
    pub fn new(name: &str) -> Self {
        IntroFlow {
            app_name: name.to_string(),
            launcher: Launcher::new(),
            controls: vec![],
        }
    }

    pub fn with_controls(mut self, controls: &[(&str, &str)]) -> Self {
        self.controls.extend(
            controls
                .iter()
                .map(|(key, action)| (key.to_string(), action.to_string())),
        );
        self
    }
}

impl<Launcher: GameLauncher> Flow for IntroFlow<Launcher> {
    fn render(&self) {
        let controls: Vec<String> = self
            .controls
            .iter()
            .map(|(keys, action)| format!("{} -> {}", keys, action))
            .collect();

        let name = format!("Tiny {}", self.app_name);
        let any_key_string = String::from("Any key to start!");

        let strings: Vec<&String> = iter::once(&name)
            .chain(&controls)
            .chain(iter::once(&any_key_string))
            .collect();

        let max_len = strings
            .iter()
            .max_by_key(|string| string.len())
            .unwrap()
            .len();

        let intro_len = max_len + 6;
        let text_len = intro_len - 2;

        let print_centered = |string: String| {
            let correction = usize::from(string.len() % 2 == 0);
            let padding = (text_len - string.len()) / 2;
            println!(
                "#{}{}{}#",
                " ".repeat(padding),
                string,
                " ".repeat(padding + correction)
            );
        };
        println!("{}", "#".repeat(intro_len));
        println!("#{}#", " ".repeat(text_len));
        print_centered(name);
        println!("#{}#", " ".repeat(text_len));
        println!("#{}#", " ".repeat(text_len));
        for control in controls {
            print_centered(control);
        }
        println!("#{}#", " ".repeat(text_len));
        print_centered(any_key_string);
        println!("#{}#", " ".repeat(text_len));
        println!("{}", "#".repeat(intro_len));
    }

    fn handle_key(&mut self, key: Key) -> Option<Box<dyn Flow>> {
        if key == Key::Q {
            return Some(Box::new(QuitFlow));
        }

        Some(self.launcher.launch_game())
    }
}

pub struct QuitFlow;
impl Flow for QuitFlow {
    fn should_quit(&self) -> bool {
        true
    }
}
