mod flows;
mod level;
mod sokoban;

use flows::SokobanLauncher;
use tiny::app::TinyApp;
use tiny::flow::IntroFlow;

fn main() {
    let controls = [
        ("wasd", "move"),
        ("r", "reset"),
        ("u", "undo"),
        ("q", "quit"),
    ];

    let mut app = TinyApp::new()
        .with_flow(IntroFlow::<SokobanLauncher>::new("Sokoban").with_controls(&controls));

    app.run();
}
