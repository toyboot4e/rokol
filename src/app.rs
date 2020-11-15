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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SAppKeycode {
    KeyInvalid = 0,
    KeySpace = 32,
    KeyApostrophe = 39,
    KeyComma = 44,
    KeyMinus = 45,
    KeyPeriod = 46,
    KeySlash = 47,
    Key0 = 48,
    Key1 = 49,
    Key2 = 50,
    Key3 = 51,
    Key4 = 52,
    Key5 = 53,
    Key6 = 54,
    Key7 = 55,
    Key8 = 56,
    Key9 = 57,
    KeySemicolon = 59,
    KeyEqual = 61,
    KeyA = 65,
    KeyB = 66,
    KeyC = 67,
    KeyD = 68,
    KeyE = 69,
    KeyF = 70,
    KeyG = 71,
    KeyH = 72,
    KeyI = 73,
    KeyJ = 74,
    KeyK = 75,
    KeyL = 76,
    KeyM = 77,
    KeyN = 78,
    KeyO = 79,
    KeyP = 80,
    KeyQ = 81,
    KeyR = 82,
    KeyS = 83,
    KeyT = 84,
    KeyU = 85,
    KeyV = 86,
    KeyW = 87,
    KeyX = 88,
    KeyY = 89,
    KeyZ = 90,
    KeyLeftBracket = 91,
    KeyBackslash = 92,
    KeyRightBracket = 93,
    KeyGraveAccent = 96,
    KeyWorld1 = 161,
    KeyWorld2 = 162,
    KeyEscape = 256,
    KeyEnter = 257,
    KeyTab = 258,
    KeyBackspace = 259,
    KeyInsert = 260,
    KeyDelete = 261,
    KeyRight = 262,
    KeyLeft = 263,
    KeyDown = 264,
    KeyUp = 265,
    KeyPageUp = 266,
    KeyPageDown = 267,
    KeyHome = 268,
    KeyEnd = 269,
    KeyCapsLock = 280,
    KeyScrollLock = 281,
    KeyNumLock = 282,
    KeyPrintScreen = 283,
    KeyPause = 284,
    KeyF1 = 290,
    KeyF2 = 291,
    KeyF3 = 292,
    KeyF4 = 293,
    KeyF5 = 294,
    KeyF6 = 295,
    KeyF7 = 296,
    KeyF8 = 297,
    KeyF9 = 298,
    KeyF10 = 299,
    KeyF11 = 300,
    KeyF12 = 301,
    KeyF13 = 302,
    KeyF14 = 303,
    KeyF15 = 304,
    KeyF16 = 305,
    KeyF17 = 306,
    KeyF18 = 307,
    KeyF19 = 308,
    KeyF20 = 309,
    KeyF21 = 310,
    KeyF22 = 311,
    KeyF23 = 312,
    KeyF24 = 313,
    KeyF25 = 314,
    KeyKP0 = 320,
    KeyKP1 = 321,
    KeyKP2 = 322,
    KeyKP3 = 323,
    KeyKP4 = 324,
    KeyKP5 = 325,
    KeyKP6 = 326,
    KeyKP7 = 327,
    KeyKP8 = 328,
    KeyKP9 = 329,
    KeyKPDecimal = 330,
    KeyKPDivide = 331,
    KeyKPMultiply = 332,
    KeyKPSubtract = 333,
    KeyKPAdd = 334,
    KeyKPEnter = 335,
    KeyKPEqual = 336,
    KeyLeftShift = 340,
    KeyLeftControl = 341,
    KeyLeftAlt = 342,
    KeyLeftSuper = 343,
    KeyRightShift = 344,
    KeyRightControl = 345,
    KeyRightAlt = 346,
    KeyRightSuper = 347,
    KeyMenu = 348,
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
    pub key_code: SAppKeycode,
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
