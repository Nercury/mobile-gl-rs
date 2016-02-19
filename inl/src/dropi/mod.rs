pub mod ffi;
pub mod glue;

use libc;
use std::ffi::CString;
use self::ffi::android::log;

/// Output log to android system.
pub fn log(prio: log::LogPriority, msg: &str) {
    let c_tag = CString::new("inl").unwrap();
    let c_text = CString::new(msg).unwrap();
    unsafe { log::__android_log_write(prio as libc::c_int, c_tag.into_raw(), c_text.into_raw()) };
}

/// Output VERBOSE log to android system.
pub fn logv(msg: &str) {
    log(log::LogPriority::VERBOSE, msg);
}

/// Output INFO log to android system.
pub fn logi(msg: &str) {
    log(log::LogPriority::INFO, msg);
}

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
