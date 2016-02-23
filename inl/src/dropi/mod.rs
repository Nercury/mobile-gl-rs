pub mod ffi;
pub mod glue;

pub enum LifecycleState {
    Started,
    Resumed,
}

pub trait Activity: Send {
    fn set_state(&mut self, state: LifecycleState);
    fn gain_focus(&mut self);
    fn loose_focus(&mut self);
    fn init_window(&mut self) -> Option<Box<Window>>;
}

pub trait Window {
    fn render(&mut self);
}
