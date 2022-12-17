use core::time;
use std::thread;

use crate::flow::{DefaultFlow, Flow};
use crate::platform::{Key, Platform};

pub struct TinyApp {
    flow: Box<dyn Flow>,
    platform: Platform,
}

impl TinyApp {
    pub fn new() -> Self {
        TinyApp {
            flow: Box::new(DefaultFlow {}),
            platform: Platform::new(),
        }
    }

    pub fn with_flow<FlowType>(mut self, flow: FlowType) -> Self
    where
        FlowType: Flow + 'static,
    {
        self.flow = Box::new(flow);
        self
    }

    pub fn run(&mut self) {
        while !self.should_quit() {
            if let Some(input_char) = self.platform.poll_input() {
                let key = Platform::translate_input(input_char);
                self.handle_key(key);
            }

            self.update();

            Platform::clear_display();
            self.render();

            thread::sleep(time::Duration::from_millis(33));
        }
    }

    fn render(&self) {
        self.flow.render();
    }

    fn handle_key(&mut self, key: Key) {
        if let Some(new_flow) = self.flow.handle_key(key) {
            self.flow = new_flow;
        }
    }

    fn update(&mut self) {
        if self.should_quit() {
            return;
        }

        if let Some(new_flow) = self.flow.update() {
            self.flow = new_flow;
        }
    }

    fn should_quit(&self) -> bool {
        self.flow.should_quit()
    }
}

impl Default for TinyApp {
    fn default() -> Self {
        Self::new()
    }
}
