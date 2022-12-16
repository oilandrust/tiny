use crate::platform::Key;

use crate::flows::DefaultFlow;

pub trait Flow {
    fn render(&self) {}

    fn update(&mut self, _key: Key) -> Option<Box<dyn Flow>> {
        None
    }

    fn should_quit(&self) -> bool {
        false
    }
}

pub struct AppFlow {
    flow: Box<dyn Flow>,
}

impl AppFlow {
    pub fn new() -> Self {
        AppFlow {
            flow: Box::new(DefaultFlow {}),
        }
    }

    pub fn with_flow<FlowType>(mut self, flow: FlowType) -> Self
    where
        FlowType: Flow + 'static,
    {
        self.flow = Box::new(flow);
        self
    }

    pub fn render(&self) {
        self.flow.render();
    }

    pub fn update(&mut self, key: Key) {
        if let Some(new_flow) = self.flow.update(key) {
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
