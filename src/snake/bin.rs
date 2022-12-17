mod flows;
mod snake;

use flows::SnakeLauncher;
use tiny::{app::TinyApp, flow::IntroFlow};

fn main() {
    let controls = [("wasd", "move"), ("q", "quit")];

    let mut app =
        TinyApp::new().with_flow(IntroFlow::<SnakeLauncher>::new("Snake").with_controls(&controls));

    app.run();
}
