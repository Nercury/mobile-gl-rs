use egli::{ self, Display };
use dropi::{ self, Activity, Window, LifecycleState };

pub struct InspectorActivity;

impl InspectorActivity {
    pub fn new(state: &[u8]) -> InspectorActivity {
        InspectorActivity
    }
}

impl Activity for InspectorActivity {
    fn set_state(&mut self, lifecycle_state: LifecycleState) {
        match lifecycle_state {
            LifecycleState::Started => dropi::logi("I was started!"),
            LifecycleState::Resumed => dropi::logi("I was resumed!"),
        }
    }

    fn gain_focus(&mut self) {
        dropi::logi("I gained focus!");
    }

    fn loose_focus(&mut self) {
        dropi::logi("I lost focus!");
    }

    fn init_window(&mut self) -> Option<Box<Window>> {
        Some(Box::new(InspectorWindow::new()))
    }
}

pub struct InspectorWindow {
    display: egli::Display,
}

impl InspectorWindow {
    pub fn new() -> InspectorWindow {
        let mut display = Display::new(egli::GetDisplay::Default)
            .expect("failed to get EGL display");

        display.initialize()
            .expect("failed to initialize EGL display");

        display.choose_config()
            .expect("failed to choose EGL display config");

        InspectorWindow {
            display: display,
        }
    }
}

impl Window for InspectorWindow {
    fn render(&mut self) {

    }
}
