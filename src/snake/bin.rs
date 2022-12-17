use core::time;
use std::thread;

use tiny::flow::AppFlow;
use tiny::flows::IntroFlow;
use tiny::platform::Platform;

mod flows;
mod snake;

use flows::{GameFlow, SnakeLauncher};

fn main() {
    let controls = [("wasd", "move"), ("q", "quit")];

    //with_flow(IntroFlow::<SnakeLauncher>::new("Snake").with_controls(&controls)
    let mut platform = Platform::new();
    let mut app = AppFlow::new().with_flow(GameFlow::new());

    while !app.should_quit() {
        Platform::clear_display();
        app.render();

        // let input_char = platform.read_char();
        // let key = Platform::translate_input(input_char);

        // app.handle_key(key);
        app.update();

        thread::sleep(time::Duration::from_millis(33));
    }
}
