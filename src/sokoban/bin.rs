mod flows;
mod level;
mod sokoban;

use flows::SokobanLauncher;
use tiny::flow::AppFlow;
use tiny::flows::IntroFlow;
use tiny::platform::Platform;

fn main() {
    let controls = [
        ("wasd", "move"),
        ("r", "reset"),
        ("u", "undo"),
        ("q", "quit"),
    ];

    let mut app = AppFlow::new()
        .with_flow(IntroFlow::<SokobanLauncher>::new("Sokoban").with_controls(&controls));

    app.run();
}
