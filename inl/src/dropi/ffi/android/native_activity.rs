use libc;
use dropi::ffi::jni::*;
use dropi::ffi::android::rect::*;
use dropi::ffi::android::native_window::*;
use dropi::ffi::android::asset_manager::*;
use dropi::ffi::android::input::*;
use dropi::ffi::android::window::*;

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct ANativeActivity {
    pub callbacks: *mut ANativeActivityCallbacks,
    pub vm: *mut JavaVM,
    pub env: *mut JNIEnv,
    pub clazz: jobject,
    pub internalDataPath: *const ::std::os::raw::c_char,
    pub externalDataPath: *const ::std::os::raw::c_char,
    pub sdkVersion: i32,
    pub instance: *mut ::std::os::raw::c_void,
    pub assetManager: *mut AAssetManager,
    pub obbPath: *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for ANativeActivity {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for ANativeActivity {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct ANativeActivityCallbacks {
    pub onStart: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                *mut ANativeActivity)>,
    pub onResume: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                 *mut ANativeActivity)>,
    pub onSaveInstanceState: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                            *mut ANativeActivity,
                                                                        outSize:
                                                                            *mut usize)
                                                       ->
                                                           *mut ::std::os::raw::c_void>,
    pub onPause: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                *mut ANativeActivity)>,
    pub onStop: ::std::option::Option<unsafe extern "C" fn(activity:
                                                               *mut ANativeActivity)>,
    pub onDestroy: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                  *mut ANativeActivity)>,
    pub onWindowFocusChanged: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                             *mut ANativeActivity,
                                                                         hasFocus:
                                                                             libc::c_int)>,
    pub onNativeWindowCreated: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                              *mut ANativeActivity,
                                                                          window:
                                                                              *mut ANativeWindow)>,
    pub onNativeWindowResized: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                              *mut ANativeActivity,
                                                                          window:
                                                                              *mut ANativeWindow)>,
    pub onNativeWindowRedrawNeeded: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                                   *mut ANativeActivity,
                                                                               window:
                                                                                   *mut ANativeWindow)>,
    pub onNativeWindowDestroyed: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                                *mut ANativeActivity,
                                                                            window:
                                                                                *mut ANativeWindow)>,
    pub onInputQueueCreated: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                            *mut ANativeActivity,
                                                                        queue:
                                                                            *mut AInputQueue)>,
    pub onInputQueueDestroyed: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                              *mut ANativeActivity,
                                                                          queue:
                                                                              *mut AInputQueue)>,
    pub onContentRectChanged: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                             *mut ANativeActivity,
                                                                         rect:
                                                                             *const ARect)>,
    pub onConfigurationChanged: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                               *mut ANativeActivity)>,
    pub onLowMemory: ::std::option::Option<unsafe extern "C" fn(activity:
                                                                    *mut ANativeActivity)>,
}
impl ::std::clone::Clone for ANativeActivityCallbacks {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for ANativeActivityCallbacks {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ANativeActivity_createFunc =
    unsafe extern "C" fn(activity: *mut ANativeActivity,
                         savedState: *mut ::std::os::raw::c_void,
                         savedStateSize: usize);
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ANativeActivityShowSoftInput {
    IMPLICIT = 1,
    FORCED = 2,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ANativeActivityHideSoftInput {
    IMPLICIT_ONLY = 1,
    NOT_ALWAYS = 2,
}
extern "C" {
    pub fn ANativeActivity_onCreate(arg1: *mut ANativeActivity,
                                    arg2: *mut ::std::os::raw::c_void,
                                    arg3: usize);
    pub fn ANativeActivity_finish(activity: *mut ANativeActivity);
    pub fn ANativeActivity_setWindowFormat(activity: *mut ANativeActivity,
                                           format: WindowFormat);
    pub fn ANativeActivity_setWindowFlags(activity: *mut ANativeActivity,
                                          addFlags: AWindowFlag,
                                          removeFlags: AWindowFlag);
    pub fn ANativeActivity_showSoftInput(activity: *mut ANativeActivity,
                                         flags: ANativeActivityShowSoftInput);
    pub fn ANativeActivity_hideSoftInput(activity: *mut ANativeActivity,
                                         flags: ANativeActivityHideSoftInput);
}
