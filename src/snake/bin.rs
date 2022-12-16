use tinylib::flow::AppFlow;
use tinylib::flows::IntroFlow;
use tinylib::platform::Platform;

mod flows;
mod snake;

use flows::SnakeLauncher;

fn main() {
    let controls = [("wasd", "move"), ("q", "quit")];

    let mut platform = Platform::new();
    let mut app = AppFlow::new()
        .start_flow(IntroFlow::<SnakeLauncher>::new("Snake").with_controls(&controls));

    while !app.should_quit() {
        Platform::clear_display();
        app.render();

        let input_char = platform.read_char();
        let key = Platform::translate_input(input_char);

        app.update(key);
    }
}
