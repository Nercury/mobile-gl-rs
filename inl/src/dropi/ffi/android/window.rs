/* automatically generated by rust-bindgen */

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum AWindowFlag {
    ALLOW_LOCK_WHILE_SCREEN_ON = 1,
    DIM_BEHIND = 2,
    BLUR_BEHIND = 4,
    NOT_FOCUSABLE = 8,
    NOT_TOUCHABLE = 16,
    NOT_TOUCH_MODAL = 32,
    TOUCHABLE_WHEN_WAKING = 64,
    KEEP_SCREEN_ON = 128,
    LAYOUT_IN_SCREEN = 256,
    LAYOUT_NO_LIMITS = 512,
    FULLSCREEN = 1024,
    FORCE_NOT_FULLSCREEN = 2048,
    DITHER = 4096,
    SECURE = 8192,
    SCALED = 16384,
    IGNORE_CHEEK_PRESSES = 32768,
    LAYOUT_INSET_DECOR = 65536,
    ALT_FOCUSABLE_IM = 131072,
    WATCH_OUTSIDE_TOUCH = 262144,
    SHOW_WHEN_LOCKED = 524288,
    SHOW_WALLPAPER = 1048576,
    TURN_SCREEN_ON = 2097152,
    DISMISS_KEYGUARD = 4194304,
}
