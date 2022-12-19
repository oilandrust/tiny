use core::time;
use std::thread;
use std::time::{Duration, Instant};

use crate::flow::{DefaultFlow, Flow};
use crate::platform::{Key, Platform};

pub struct Time {
    pub frame_delta_time: Duration,
    pub time_since_startup: Duration,
}

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
        const FRAME_TIME_TARGET: time::Duration = time::Duration::from_millis(33);
        let game_began = Instant::now();
        let mut last_frame_time = Instant::now().duration_since(game_began);

        while !self.should_quit() {
            let time_now = Instant::now().duration_since(game_began);
            let elapsed_time = time_now - last_frame_time;
            last_frame_time = time_now;

            if let Some(input_char) = self.platform.poll_input() {
                let key = Platform::translate_input(input_char);
                self.handle_key(key);
            }

            self.update(&Time {
                frame_delta_time: elapsed_time,
                time_since_startup: time_now,
            });

            Platform::clear_display();
            self.render();

            if elapsed_time < FRAME_TIME_TARGET {
                thread::sleep(FRAME_TIME_TARGET - elapsed_time);
            }
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

    fn update(&mut self, time: &Time) {
        if self.should_quit() {
            return;
        }

        if let Some(new_flow) = self.flow.update(time) {
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
