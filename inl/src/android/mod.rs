use egli::{ self, surface, renderable, Display, ContextClientVersion };
use dropi::{ Activity, Window, LifecycleState };
use dropi::error::Result;
use dropi::ffi::android::native_window::ANativeWindow;
use std::mem;
use android_sensor::{ SensorManagerRef, SensorEventQueue, SensorType };
use android_looper::{ LooperRef, LooperPrepareOpts };

pub struct InspectorActivity;

impl InspectorActivity {
    pub fn new(_state: &[u8]) -> InspectorActivity {
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
        info!("I got focus!");
    }

    fn loose_focus(&mut self) {
        info!("I lost focus!");
    }

    fn create_window(&mut self, window_handle: *mut ANativeWindow) -> Result<Box<Window>> {
        Ok(Box::new(InspectorWindow::new(window_handle)?))
    }

    fn init(&mut self, _window: &mut Window) {

    }

    fn render(&mut self, window: &mut Window) {
        let pos = window.update().expect("Update failed");

        info!("acc {:?}", pos);
    }
}

pub struct InspectorWindow {
    window_size: (i32, i32),
    _context: egli::Context,
    surface: egli::Surface,
    display: egli::Display,
    looper: LooperRef,
    accelerometer_event_queue: SensorEventQueue,
    sensor_data_filter: (f32, f32, f32),
}

const LOOPER_ID_USER: i32 = 3;
const SENSOR_REFRESH_RATE: i32 = 100;
const SENSOR_FILTER_ALPHA: f32 = 0.1;

impl InspectorWindow {
    pub fn new(window_handle: *mut ANativeWindow) -> Result<InspectorWindow> {
        let looper = LooperRef::prepare(LooperPrepareOpts::AllowNonCallbacks)
            .expect("Failed to prepare looper");

        let sensor_manager = SensorManagerRef::get().expect("Failed to get SensorManager");

        let accelerometer = sensor_manager.get_default_sensor(SensorType::Accelerometer)
            .expect("Failed to get accelerometer");
        let accelerometer_event_queue = sensor_manager.create_event_queue(looper, LOOPER_ID_USER)
            .expect("Failed to create event queue");
        let _ = accelerometer_event_queue.set_event_rate(accelerometer, 1000000 / SENSOR_REFRESH_RATE);
        accelerometer_event_queue.enable_sensor(accelerometer)
            .expect("Error enabling sensor");

        let display = Display::from_default_display()
            .expect("failed to get EGL display");

        let egl_version = display.initialize_and_get_version()
            .expect("failed to initialize EGL");
        info!("Using EGL2 {}", egl_version);

        let configs = display.config_filter()
            .with_surface_type(surface::WINDOW)
            .with_renderable_type(renderable::OPENGL_ES2)
            .with_blue_size(8)
            .with_green_size(8)
            .with_red_size(8)
            .with_depth_size(24)
            .choose_configs()
            .expect("failed to get display configs");

        let surface = display.create_window_surface(*configs.first().unwrap(), unsafe { mem::transmute(window_handle) })?;
        let context = display.create_context_with_client_version(*configs.first().unwrap(), ContextClientVersion::OpenGlEs2)?;

        display.make_current(&surface, &surface, &context)?;

        // gl::load_with(|s| unsafe { mem::transmute(egli::egl::get_proc_address(s)) });

        Ok(InspectorWindow {
            window_size: (surface.query_width()?, surface.query_height()?),
            display: display,
            surface: surface,
            _context: context,
            looper: looper,
            accelerometer_event_queue: accelerometer_event_queue,
            sensor_data_filter: (0.0, 0.0, 0.0),
        })
    }
}

impl Window for InspectorWindow {
    fn size(&self) -> (i32, i32) {
        self.window_size
    }

    fn swap_buffers(&mut self) -> Result<()> {
        self.display.swap_buffers(&self.surface)?;
        Ok(())
    }

    fn update(&mut self) -> Result<(f32, f32, f32)> {
        self.looper.poll_all_blind();

        let a = SENSOR_FILTER_ALPHA;

        for event in &mut self.accelerometer_event_queue {
            let acc = event.acceleration().unwrap();

            self.sensor_data_filter.0 = a * acc.0 + (1.0 - a) * self.sensor_data_filter.0;
            self.sensor_data_filter.1 = a * acc.1 + (1.0 - a) * self.sensor_data_filter.1;
            self.sensor_data_filter.2 = a * acc.2 + (1.0 - a) * self.sensor_data_filter.2;
        }

        Ok(self.sensor_data_filter)
    }
}
