use rokol_ffi::app as ffi;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SAppEventType {
    Invalid,
    KeyDown,
    KeyUp,
    Char,
    MouseDown,
    MouseUp,
    MouseScroll,
    MouseMove,
    MouseEnter,
    MouseLeave,
    TouchesBegan,
    TouchesMoved,
    TouchesEnded,
    TouchesCancelled,
    Resized,
    Iconified,
    Restored,
    Suspended,
    Resumed,
    UpdateCursor,
    QuitRequested,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum Key {
    Invalid = ffi::sapp_keycode_SAPP_KEYCODE_INVALID,
    Space = ffi::sapp_keycode_SAPP_KEYCODE_SPACE,
    Apostrophe = ffi::sapp_keycode_SAPP_KEYCODE_APOSTROPHE,
    Comma = ffi::sapp_keycode_SAPP_KEYCODE_COMMA,
    Minus = ffi::sapp_keycode_SAPP_KEYCODE_MINUS,
    Period = ffi::sapp_keycode_SAPP_KEYCODE_PERIOD,
    Slash = ffi::sapp_keycode_SAPP_KEYCODE_SLASH,
    Kbd0 = ffi::sapp_keycode_SAPP_KEYCODE_0,
    Kbd1 = ffi::sapp_keycode_SAPP_KEYCODE_1,
    Kbd2 = ffi::sapp_keycode_SAPP_KEYCODE_2,
    Kbd3 = ffi::sapp_keycode_SAPP_KEYCODE_3,
    Kbd4 = ffi::sapp_keycode_SAPP_KEYCODE_4,
    Kbd5 = ffi::sapp_keycode_SAPP_KEYCODE_5,
    Kbd6 = ffi::sapp_keycode_SAPP_KEYCODE_6,
    Kbd7 = ffi::sapp_keycode_SAPP_KEYCODE_7,
    Kbd8 = ffi::sapp_keycode_SAPP_KEYCODE_8,
    Kbd9 = ffi::sapp_keycode_SAPP_KEYCODE_9,
    Semicolon = ffi::sapp_keycode_SAPP_KEYCODE_SEMICOLON,
    Equal = ffi::sapp_keycode_SAPP_KEYCODE_EQUAL,
    A = ffi::sapp_keycode_SAPP_KEYCODE_A,
    B = ffi::sapp_keycode_SAPP_KEYCODE_B,
    C = ffi::sapp_keycode_SAPP_KEYCODE_C,
    D = ffi::sapp_keycode_SAPP_KEYCODE_D,
    E = ffi::sapp_keycode_SAPP_KEYCODE_E,
    F = ffi::sapp_keycode_SAPP_KEYCODE_F,
    G = ffi::sapp_keycode_SAPP_KEYCODE_G,
    H = ffi::sapp_keycode_SAPP_KEYCODE_H,
    I = ffi::sapp_keycode_SAPP_KEYCODE_I,
    J = ffi::sapp_keycode_SAPP_KEYCODE_J,
    K = ffi::sapp_keycode_SAPP_KEYCODE_K,
    L = ffi::sapp_keycode_SAPP_KEYCODE_L,
    M = ffi::sapp_keycode_SAPP_KEYCODE_M,
    N = ffi::sapp_keycode_SAPP_KEYCODE_N,
    O = ffi::sapp_keycode_SAPP_KEYCODE_O,
    P = ffi::sapp_keycode_SAPP_KEYCODE_P,
    Q = ffi::sapp_keycode_SAPP_KEYCODE_Q,
    R = ffi::sapp_keycode_SAPP_KEYCODE_R,
    S = ffi::sapp_keycode_SAPP_KEYCODE_S,
    T = ffi::sapp_keycode_SAPP_KEYCODE_T,
    U = ffi::sapp_keycode_SAPP_KEYCODE_U,
    V = ffi::sapp_keycode_SAPP_KEYCODE_V,
    W = ffi::sapp_keycode_SAPP_KEYCODE_W,
    X = ffi::sapp_keycode_SAPP_KEYCODE_X,
    Y = ffi::sapp_keycode_SAPP_KEYCODE_Y,
    Z = ffi::sapp_keycode_SAPP_KEYCODE_Z,
    LeftBracket = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET,
    Backslash = ffi::sapp_keycode_SAPP_KEYCODE_BACKSLASH,
    RightBracket = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET,
    GraveAccent = ffi::sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT,
    World1 = ffi::sapp_keycode_SAPP_KEYCODE_WORLD_1,
    World2 = ffi::sapp_keycode_SAPP_KEYCODE_WORLD_2,
    Escape = ffi::sapp_keycode_SAPP_KEYCODE_ESCAPE,
    Enter = ffi::sapp_keycode_SAPP_KEYCODE_ENTER,
    Tab = ffi::sapp_keycode_SAPP_KEYCODE_TAB,
    Backspace = ffi::sapp_keycode_SAPP_KEYCODE_BACKSPACE,
    Insert = ffi::sapp_keycode_SAPP_KEYCODE_INSERT,
    Delete = ffi::sapp_keycode_SAPP_KEYCODE_DELETE,
    Right = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT,
    Left = ffi::sapp_keycode_SAPP_KEYCODE_LEFT,
    Down = ffi::sapp_keycode_SAPP_KEYCODE_DOWN,
    Up = ffi::sapp_keycode_SAPP_KEYCODE_UP,
    PageUp = ffi::sapp_keycode_SAPP_KEYCODE_PAGE_UP,
    PageDown = ffi::sapp_keycode_SAPP_KEYCODE_PAGE_DOWN,
    Home = ffi::sapp_keycode_SAPP_KEYCODE_HOME,
    End = ffi::sapp_keycode_SAPP_KEYCODE_END,
    CapsLock = ffi::sapp_keycode_SAPP_KEYCODE_CAPS_LOCK,
    ScrollLock = ffi::sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK,
    NumLock = ffi::sapp_keycode_SAPP_KEYCODE_NUM_LOCK,
    PrintScreen = ffi::sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN,
    Pause = ffi::sapp_keycode_SAPP_KEYCODE_PAUSE,
    F1 = ffi::sapp_keycode_SAPP_KEYCODE_F1,
    F2 = ffi::sapp_keycode_SAPP_KEYCODE_F2,
    F3 = ffi::sapp_keycode_SAPP_KEYCODE_F3,
    F4 = ffi::sapp_keycode_SAPP_KEYCODE_F4,
    F5 = ffi::sapp_keycode_SAPP_KEYCODE_F5,
    F6 = ffi::sapp_keycode_SAPP_KEYCODE_F6,
    F7 = ffi::sapp_keycode_SAPP_KEYCODE_F7,
    F8 = ffi::sapp_keycode_SAPP_KEYCODE_F8,
    F9 = ffi::sapp_keycode_SAPP_KEYCODE_F9,
    F10 = ffi::sapp_keycode_SAPP_KEYCODE_F10,
    F11 = ffi::sapp_keycode_SAPP_KEYCODE_F11,
    F12 = ffi::sapp_keycode_SAPP_KEYCODE_F12,
    F13 = ffi::sapp_keycode_SAPP_KEYCODE_F13,
    F14 = ffi::sapp_keycode_SAPP_KEYCODE_F14,
    F15 = ffi::sapp_keycode_SAPP_KEYCODE_F15,
    F16 = ffi::sapp_keycode_SAPP_KEYCODE_F16,
    F17 = ffi::sapp_keycode_SAPP_KEYCODE_F17,
    F18 = ffi::sapp_keycode_SAPP_KEYCODE_F18,
    F19 = ffi::sapp_keycode_SAPP_KEYCODE_F19,
    F20 = ffi::sapp_keycode_SAPP_KEYCODE_F20,
    F21 = ffi::sapp_keycode_SAPP_KEYCODE_F21,
    F22 = ffi::sapp_keycode_SAPP_KEYCODE_F22,
    F23 = ffi::sapp_keycode_SAPP_KEYCODE_F23,
    F24 = ffi::sapp_keycode_SAPP_KEYCODE_F24,
    F25 = ffi::sapp_keycode_SAPP_KEYCODE_F25,
    KP0 = ffi::sapp_keycode_SAPP_KEYCODE_KP_0,
    KP1 = ffi::sapp_keycode_SAPP_KEYCODE_KP_1,
    KP2 = ffi::sapp_keycode_SAPP_KEYCODE_KP_2,
    KP3 = ffi::sapp_keycode_SAPP_KEYCODE_KP_3,
    KP4 = ffi::sapp_keycode_SAPP_KEYCODE_KP_4,
    KP5 = ffi::sapp_keycode_SAPP_KEYCODE_KP_5,
    KP6 = ffi::sapp_keycode_SAPP_KEYCODE_KP_6,
    KP7 = ffi::sapp_keycode_SAPP_KEYCODE_KP_7,
    KP8 = ffi::sapp_keycode_SAPP_KEYCODE_KP_8,
    KP9 = ffi::sapp_keycode_SAPP_KEYCODE_KP_9,
    KPDecimal = ffi::sapp_keycode_SAPP_KEYCODE_KP_DECIMAL,
    KPDivide = ffi::sapp_keycode_SAPP_KEYCODE_KP_DIVIDE,
    KPMultiply = ffi::sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY,
    KPSubtract = ffi::sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT,
    KPAdd = ffi::sapp_keycode_SAPP_KEYCODE_KP_ADD,
    KPEnter = ffi::sapp_keycode_SAPP_KEYCODE_KP_ENTER,
    KPEqual = ffi::sapp_keycode_SAPP_KEYCODE_KP_EQUAL,
    LeftShift = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT,
    LeftControl = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL,
    LeftAlt = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_ALT,
    LeftSuper = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_SUPER,
    RightShift = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT,
    RightControl = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL,
    RightAlt = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_ALT,
    RightSuper = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER,
    Menu = ffi::sapp_keycode_SAPP_KEYCODE_MENU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SAppMouseButton {
    Invalid = -1,
    Left = 0,
    Right = 1,
    Middle = 2,
}

bitflags! {
    #[repr(C)]
    pub struct SAppModifier: u32 {
        const SHIFT = 0x01;
        const CONTROL = 0x02;
        const ALT = 0x04;
        const SUPER = 0x08;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SAppTouchPoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}

#[derive(Debug)]
pub struct SAppEvent {
    pub frame_count: u64,
    pub event_type: SAppEventType,
    pub key_code: Key,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: SAppModifier,
    pub mouse_button: SAppMouseButton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: i32,
    pub touches: [SAppTouchPoint; ffi::SAPP_MAX_TOUCHPOINTS],
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
}

#[derive(Default, Debug)]
pub struct SAppDesc {
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: String,
    pub user_cursor: bool,

    pub html5_canvas_name: String,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}

pub trait SApp {
    /// Init callback function.
    fn sapp_init(&mut self);

    /// Frame callback function.
    fn sapp_frame(&mut self);

    /// Cleanup callback function.
    fn sapp_cleanup(&mut self);

    /// Event callback function.
    fn sapp_event(&mut self, event: SAppEvent);

    /// Optional `sokol_app` error reporting callback function.
    fn sapp_fail(&mut self, msg: &str) {
        print!("{}", msg);
    }

    /// Function called by `sokol_audio` in callback mode.
    ///
    /// The default implementation clears the buffer to zero. Applications
    /// using this mode are expected to mix audio data into the buffer.
    ///
    /// This is called from a separate thread on all desktop platforms.
    fn saudio_stream(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32) {
        let len = (num_frames * num_channels) as usize;
        for i in 0..len {
            buffer[i] = 0.0;
        }
    }
}

pub struct SAppImpl {
    callbacks: Box<SApp>,
    desc: SAppDesc,
}

impl SAppImpl {
    fn new<S: SApp + 'static>(callbacks: S, desc: SAppDesc) -> SAppImpl {
        SAppImpl {
            callbacks: Box::new(callbacks),
            desc,
        }
    }

    pub fn init_cb(&mut self) {
        self.callbacks.sapp_init();
    }

    pub fn frame_cb(&mut self) {
        self.callbacks.sapp_frame();
    }

    pub fn cleanup_cb(&mut self) {
        self.callbacks.sapp_cleanup();
    }

    pub fn event_cb(&mut self, event: SAppEvent) {
        self.callbacks.sapp_event(event);
    }

    pub fn fail_cb(&mut self, msg: &str) {
        self.callbacks.sapp_fail(msg);
    }

    pub fn stream_cb(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32) {
        self.callbacks
            .saudio_stream(buffer, num_frames, num_channels);
    }

    pub fn get(user_data: *mut c_void) -> &'static mut SAppImpl {
        unsafe {
            let app_ptr = user_data as *mut SAppImpl;
            &mut *app_ptr
        }
    }
}

pub fn sapp_run<S: SApp + 'static>(callbacks: S, desc: SAppDesc) -> i32 {
    let app = SAppImpl::new(callbacks, desc);

    unsafe { ffi::sapp_run(&ffi::sapp_make_desc(&app)) }
}

pub fn sapp_isvalid() -> bool {
    unsafe { ffi::sapp_isvalid() }
}

pub fn sapp_width() -> i32 {
    unsafe { ffi::sapp_width() }
}

pub fn sapp_height() -> i32 {
    unsafe { ffi::sapp_height() }
}

pub fn sapp_high_dpi() -> bool {
    unsafe { ffi::sapp_high_dpi() }
}

pub fn sapp_dpi_scale() -> f32 {
    unsafe { ffi::sapp_dpi_scale() }
}

pub fn sapp_show_keyboard(visible: bool) {
    unsafe {
        ffi::sapp_show_keyboard(visible);
    }
}

pub fn sapp_keyboard_shown() -> bool {
    unsafe { ffi::sapp_keyboard_shown() }
}

pub fn sapp_request_quit() {
    unsafe {
        ffi::sapp_request_quit();
    }
}

pub fn sapp_cancel_quit() {
    unsafe {
        ffi::sapp_cancel_quit();
    }
}

pub fn sapp_quit() {
    unsafe {
        ffi::sapp_quit();
    }
}

pub fn sapp_frame_count() -> u64 {
    unsafe { ffi::sapp_frame_count() }
}

pub fn sapp_gles2() -> bool {
    unsafe { ffi::sapp_gles2() }
}
