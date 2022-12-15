use tinylib::flow::AppFlow;
use tinylib::platform::Platform;

mod flow;
mod sokoban;

use flow::IntroFlow;
use sokoban::translate_input;

fn main() {
    let mut platform = Platform::new();
    let mut app = AppFlow::new().start_flow(IntroFlow {});

    while !app.should_quit() {
        Platform::clear_display();
        app.render();

        let input_char = platform.read_char();
        let command = translate_input(input_char);

        app.update(command);
    }
}
