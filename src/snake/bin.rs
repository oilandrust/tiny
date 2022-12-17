mod flows;
mod snake;

use flows::{GameFlow, SnakeLauncher};
use tiny::flow::AppFlow;

fn main() {
    let controls = [("wasd", "move"), ("q", "quit")];

    //with_flow(IntroFlow::<SnakeLauncher>::new("Snake").with_controls(&controls)
    let mut app = AppFlow::new().with_flow(GameFlow::new());

    app.run();
}
