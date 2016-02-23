use libc;
use std::mem;
use std::thread;
use std::time;
use std::sync::mpsc::{ self, channel };
use dropi::{ self, Activity, Window, LifecycleState };
use dropi::ffi::android::native_activity::ANativeActivity;
use dropi::ffi::android::native_window::ANativeWindow;

enum Command {
    Destroy,
    Lifecycle(LifecycleState),
    GainedFocus,
    LostFocus,
    SetWindow(Option<WindowWrapper>),
}

struct WindowWrapper {
    native: *mut ANativeWindow,
}

unsafe impl Send for WindowWrapper {}

struct InstanceWrapper {
    queue: mpsc::Sender<Command>,
    thread: thread::JoinHandle<Box<Activity>>,
}

struct WindowState {
    native: Option<WindowWrapper>,
    app: Option<Box<Window>>,
}

pub fn bind_activity_lifecycle(activity: *mut ANativeActivity, mut instance: Box<Activity>) {
    trace!("bind activity lifecycle");

    let (tx, rx) = channel();

    let activity_thread = thread::spawn(move || {
        let mut window_state = WindowState {
            native: None,
            app: None,
        };

        'main: loop {
            while let Ok(command) = rx.try_recv() {
                match command {
                    Command::Destroy => break 'main,
                    Command::Lifecycle(state) => {
                        instance.set_state(state);
                    },
                    Command::GainedFocus => instance.gain_focus(),
                    Command::LostFocus => instance.loose_focus(),
                    Command::SetWindow(maybe_window) => {
                        window_state = match (window_state.native, maybe_window) {
                            (_, Some(new)) => WindowState {
                                native: Some(new), app: instance.init_window()
                            },
                            _ =>  WindowState { native: None, app: None },
                        };
                    },
                }
            }

            if let Some(ref mut window) = window_state.app {
                window.render();
            }

            thread::sleep(time::Duration::from_millis(1));
        }

        instance
    });

    // it's less scary with single unsafe block
    unsafe {
        let callbacks = (*activity).callbacks;
        (*callbacks).onDestroy = Some(mem::transmute(on_destroy));
        (*callbacks).onStart = Some(mem::transmute(on_start));
        (*callbacks).onResume = Some(mem::transmute(on_resume));
        (*callbacks).onWindowFocusChanged = Some(mem::transmute(on_window_focus_changed));
        (*callbacks).onNativeWindowCreated = Some(mem::transmute(on_native_window_created));
        (*callbacks).onNativeWindowDestroyed = Some(mem::transmute(on_native_window_destroyed));

        (*activity).instance = mem::transmute(Box::new(InstanceWrapper {
            queue: tx,
            thread: activity_thread,
        }));
    }
}

fn take_wrapper(activity: *mut ANativeActivity) -> Box<InstanceWrapper> {
    unsafe { mem::transmute((*activity).instance) }
}

fn get_wrapper_ref<'r>(activity: *mut ANativeActivity) -> &'r mut Box<InstanceWrapper> {
    unsafe { mem::transmute(&mut (*activity).instance) }
}

extern "C" fn on_destroy(activity: *mut ANativeActivity) {
    trace!("on activity destroy");

    let wrapper: Box<InstanceWrapper> = take_wrapper(activity);

    wrapper.queue.send(Command::Destroy)
        .expect("failed to send on_destroy to activity");
    wrapper.thread.join()
        .expect("failed to join activity thread");

    trace!("cleaned up successfuly");
}

extern "C" fn on_start(activity: *mut ANativeActivity) {
    trace!("on activity start");

    get_wrapper_ref(activity)
        .queue.send(Command::Lifecycle(LifecycleState::Started))
        .expect("failed to send on_start to activity");
}

extern "C" fn on_resume(activity: *mut ANativeActivity) {
    trace!("on activity resume");

    get_wrapper_ref(activity)
        .queue.send(Command::Lifecycle(LifecycleState::Resumed))
        .expect("failed to send on_resume to activity");
}

extern "C" fn on_window_focus_changed(activity: *mut ANativeActivity, has_focus: libc::c_int) {
    trace!("on window focus changed");

    get_wrapper_ref(activity)
        .queue.send(if has_focus != 0 { Command::GainedFocus } else { Command::LostFocus })
        .expect("failed to send on_window_focus_changed to activity");
}

extern "C" fn on_native_window_created(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    trace!("on native window created");

    get_wrapper_ref(activity)
        .queue.send(Command::SetWindow(Some(WindowWrapper { native: window })))
        .expect("failed to send on_native_window_created to activity");
}

extern "C" fn on_native_window_destroyed(activity: *mut ANativeActivity, _window: *mut ANativeWindow) {
    trace!("on native window destroyed");

    get_wrapper_ref(activity)
        .queue.send(Command::SetWindow(None))
        .expect("failed to send on_native_window_destroyed to activity");
}
