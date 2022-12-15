use flow::AppFlow;
use platform::Platform;
use sokoban::translate_input;

mod flow;
mod platform;
pub mod sokoban;

fn main() {
    let mut platform = Platform::new();
    let mut app = AppFlow::new();

    while !app.should_quit() {
        Platform::clear_display();
        app.render();

        let input_char = platform.read_char();
        let command = translate_input(input_char);

        app.update(command);
    }
}
