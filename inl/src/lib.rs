#![feature(panic_handler, unique, question_mark)]

extern crate libc;
extern crate egli;
#[macro_use] extern crate log;
extern crate android_logger;
extern crate android_looper_sys;
extern crate android_looper;
extern crate android_sensor;

pub mod dropi;
pub mod android;

use std::mem;
use std::ptr;
use dropi::ffi::android::native_activity::ANativeActivity;

#[no_mangle]
pub extern "C" fn android_activity_create(
    activity: *mut ANativeActivity,
    saved_state: *mut u8,
    saved_state_size: usize
) {
    android_logger::init_once(log::LogLevel::Debug);
    log_panics();

    let state: &[u8] = unsafe { ::std::slice::from_raw_parts(saved_state, saved_state_size) };
    let activity_ptr = unsafe { ptr::Unique::<libc::c_void>::new(mem::transmute(activity)) };

    trace!("starting android_activity_create");

    very_long_module_name_which_will_not_fit::do_stuff();

    let activity: *mut ANativeActivity = unsafe { mem::transmute(*activity_ptr) };

    dropi::glue::bind_activity_lifecycle(
       activity,
       Box::new(android::InspectorActivity::new(state))
   ).expect("failed to initialize activity");
}

mod very_long_module_name_which_will_not_fit {
    pub fn do_stuff() {
        trace!("message 8");
    }
}

/// Registers a panic handler which logs at the error level.
///
/// The format is the same as the default panic handler. The reporting module is
/// `panic_logger`.
///
/// Requires the `use_std` (enabled by default) and `nightly` features.
fn log_panics() {
    std::panic::set_hook(Box::new(panic_logger::log));
}

// inner module so that the reporting module is log::panic instead of log
mod panic_logger {
    use std::panic::PanicInfo;
    use std::thread;

    pub fn log(info: &PanicInfo) {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("<unnamed>");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            }
        };

        match info.location() {
            Some(location) => {
                error!("thread '{}' panicked at '{}': {}:{}",
                       thread,
                       msg,
                       location.file(),
                       location.line())
            }
            None => error!("thread '{}' panicked at '{}'", thread, msg),
        }
    }
}
