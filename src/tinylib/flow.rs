#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Move(i32, i32),
    RestartLevel,
    Quit,
    Undo,
    Unknown,
}
pub trait Flow {
    fn render(&self) {}

    fn update(&mut self, _command: Command) -> Option<Box<dyn Flow>> {
        None
    }

    fn should_quit(&self) -> bool {
        false
    }
}

pub struct AppFlow {
    flow: Box<dyn Flow>,
}

struct DefaultFlow;
impl Flow for DefaultFlow {}

impl AppFlow {
    pub fn new() -> Self {
        AppFlow {
            flow: Box::new(DefaultFlow {}),
        }
    }

    pub fn start_flow<FlowType>(mut self, flow: FlowType) -> Self
    where
        FlowType: Flow + 'static,
    {
        self.flow = Box::new(flow);
        self
    }

    pub fn render(&self) {
        self.flow.render();
    }

    pub fn update(&mut self, command: Command) {
        if let Some(new_flow) = self.flow.update(command) {
            self.flow = new_flow;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.flow.should_quit()
    }
}

impl Default for AppFlow {
    fn default() -> Self {
        Self::new()
    }
}
