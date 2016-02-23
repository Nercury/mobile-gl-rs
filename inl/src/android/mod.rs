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
            LifecycleState::Started => info!("I was started!"),
            LifecycleState::Resumed => info!("I was resumed!"),
        }
    }

    fn gain_focus(&mut self) {
        info!("I gained focus! ");
    }

    fn loose_focus(&mut self) {
        info!("I lost focus!");
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
        let mut display = Display::from_default_display()
            .expect("failed to get EGL display");

        let egl_version = display.initialize_and_get_version()
            .expect("failed to initialize EGL");
        info!("Using EGL {}", egl_version);

        let configs = display.get_configs()
            .expect("failed to get display configs");
        info!("There are {} possible display configurations:", configs.len());
        info!("{:?}", configs);


        // display.choose_config()
        //     .expect("failed to choose EGL display config");

        InspectorWindow {
            display: display,
        }
    }
}

impl Window for InspectorWindow {
    fn render(&mut self) {

    }
}
