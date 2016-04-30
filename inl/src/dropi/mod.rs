use dropi::ffi::android::native_window::ANativeWindow;
use dropi::error::Result;

pub mod ffi;
pub mod glue;
pub mod error;

#[derive(Debug)]
pub enum LifecycleState {
    Started,
    Resumed,
}

pub trait Activity: Send {
    fn set_state(&mut self, state: LifecycleState);
    fn gain_focus(&mut self);
    fn loose_focus(&mut self);
    fn create_window(&mut self, window_handle: *mut ANativeWindow) -> Result<Box<Window>>;
    fn init(&mut self, window: &mut Window);
    fn render(&mut self, window: &mut Window);
}

pub trait Window {
    fn size(&self) -> (i32, i32);
    fn swap_buffers(&mut self) -> Result<()>;
    fn update(&mut self) -> Result<(f32, f32, f32)>;
}

#[derive(Copy, Clone, Debug)]
struct WindowWrapper {
    native: *mut ANativeWindow,
}

unsafe impl Send for WindowWrapper {}

#[derive(Debug)]
enum Command {
    Destroy,
    Lifecycle(LifecycleState),
    GainedFocus,
    LostFocus,
    SetWindow(Option<WindowWrapper>),
}
