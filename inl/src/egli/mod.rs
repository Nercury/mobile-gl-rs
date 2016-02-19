extern crate egl;

use std::mem;
use self::egl::egl as ni;

pub struct Display {
    native: ni::EGLDisplay,
}

#[repr(usize)]
pub enum GetDisplay {
    Default = 0,
}

impl Drop for Display {
    fn drop(&mut self) {
        ni::Terminate(self.native);
    }
}

impl Display {
    pub fn new(kind: GetDisplay) -> Result<Display, Error> {
        let native = ni::GetDisplay(unsafe { mem::transmute(kind) });

        if native.is_null() {
            return Err(Error::FailedToGetDisplay);
        }

        Ok(Display {
            native: native,
        })
    }

    pub fn initialize(&mut self) -> Result<(), ()> {
        let mut major = 0;
        let mut minor = 0;
        ni::Initialize(self.native, &mut major, &mut minor);
        Ok(())
    }

    pub fn choose_config(&mut self) -> Result<(), ()> {
        let config: [ni::EGLint; 9] = [
            ni::EGL_SURFACE_TYPE as ni::EGLint, ni::EGL_WINDOW_BIT as ni::EGLint,
            ni::EGL_BLUE_SIZE as ni::EGLint, 8,
            ni::EGL_GREEN_SIZE as ni::EGLint, 8,
            ni::EGL_RED_SIZE as ni::EGLint, 8,
            ni::EGL_NONE as ni::EGLint
        ];

        let mut num_config: ni::EGLint = 0;
        let mut configs: ni::EGLConfig = unsafe { mem::uninitialized() };

        ni::ChooseConfig(self.native, config.as_ptr(), &mut configs, 1, &mut num_config);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    FailedToGetDisplay,
}
