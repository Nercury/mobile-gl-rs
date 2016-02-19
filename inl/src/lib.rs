extern crate libc;

pub mod egli;
pub mod dropi;
pub mod android;
pub mod c_api;
pub use c_api::*;

use dropi::ffi::android::native_activity::ANativeActivity;

#[no_mangle]
pub extern "C" fn android_activity_create(
    activity: *mut ANativeActivity,
    saved_state: *mut u8,
    saved_state_size: usize
) {
    let state: &[u8] = unsafe { ::std::slice::from_raw_parts(saved_state, saved_state_size) };

    dropi::logv("starting android_activity_create");
    dropi::glue::bind_activity_lifecycle(
        activity,
        Box::new(android::InspectorActivity::new(state))
    );
}
