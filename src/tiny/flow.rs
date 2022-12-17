use core::time;
use std::thread;

use crate::flows::DefaultFlow;
use crate::platform::{Key, Platform};

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

pub struct AppFlow {
    flow: Box<dyn Flow>,
    platform: Platform,
}

impl AppFlow {
    pub fn new() -> Self {
        AppFlow {
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
            Platform::clear_display();
            self.render();

            if let Some(input_char) = self.platform.poll_input() {
                let key = Platform::translate_input(input_char);
                self.handle_key(key);
            }

            self.update();

            thread::sleep(time::Duration::from_millis(33));
        }
    }

    pub fn render(&self) {
        self.flow.render();
    }

    pub fn handle_key(&mut self, key: Key) {
        if let Some(new_flow) = self.flow.handle_key(key) {
            self.flow = new_flow;
        }
    }

    pub fn update(&mut self) {
        if self.should_quit() {
            return;
        }

        if let Some(new_flow) = self.flow.update() {
            self.flow = new_flow;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.flow.should_quit()
    }
}

impl Default for AppFlow {
    fn default() -> Self {
        Self::new()
    }
}
