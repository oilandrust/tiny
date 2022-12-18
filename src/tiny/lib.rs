pub mod app;
pub mod flow;
pub mod math;
pub mod platform;

pub mod prelude {
    pub use crate::app::TinyApp;
    pub use crate::flow::Flow;
    pub use crate::math::{Direction, Position};
    pub use crate::platform::Key;
    pub use std::time::Duration;
}
