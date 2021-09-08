/*!
Application ([`FFI`])

[`FFI`]: rokol_ffi::app

[`Rokol::run`](crate::glue::sapp::Rokol::run) runs an implementation of [`RApp`].
*/

use {
    bitflags::bitflags,
    rokol_ffi::app as ffi,
    std::{
        ffi::{c_void, CStr, CString},
        os::raw::c_char,
    },
};

// --------------------------------------------------------------------------------
// Hidden items

// See [`crate::Builder`]

// /// `rokol::app` application description
// pub type RAppDesc = ffi::sapp_desc;

// /// Special run-function for `SOKOL_NO_ENTRY` (in standard mode this is an empty stub)
// pub fn sapp_run(const sapp_desc* desc);

// --------------------------------------------------------------------------------
// Primary trait

/// [`rokol::app`] callbacks
///
/// [`rokol::app`]: crate::app
///
/// All provided function callbacks will be called from the same thread,
/// but this may be different from the thread where `main` was called.
pub trait RApp {
    /// Called once after the rendering surface, 3D API and swap chain have been initialized by
    /// `sokol_app`
    fn init(&mut self) {}

    /// Usually called 60 times per second
    fn frame(&mut self) {}

    /// Called once after the user quits the application
    ///
    /// Suitable place to implement "really quit?" diaglog.
    ///
    /// The cleanup-callback isn't guaranteed to be called on the web and mobile platforms.
    #[cfg(feature = "impl-gfx")]
    fn cleanup(&mut self) {
        unsafe {
            rokol_ffi::gfx::sg_shutdown();
        }
    }
    #[cfg(not(feature = "impl-gfx"))]
    fn cleanup(&mut self);

    /// Event handling
    ///
    /// Do *not* call any 3D API rendering functions in the event
    /// callback function, since the 3D API context may not be active when the
    /// event callback is called (it may work on some platforms and 3D APIs,
    /// but not others, and the exact behaviour may change between
    /// sokol-app versions).
    fn event(&mut self, _ev: &Event) {}

    /// Called when a fatal error is encountered during start which doesn't allow the program to
    /// continue
    ///
    /// Providing a callback here gives you a chance to show an error message
    /// to the user. The default behaviour is SOKOL_LOG(msg)
    fn fail(&mut self, msg: &str) {
        eprint!("{}", msg);
    }

    // --------------------------------------------------------------------------------
    // C callback functions set to [`RAppDesc`]
}

/// [`rokol::app`] callbacks for C
///
/// [`rokol::app`]: crate::app
///
/// It's makes [`RApp`] a normal rusty trait. It's implemented and used under the hood.
pub trait RAppFfiCallback {
    #[no_mangle]
    extern "C" fn init_userdata_cb(user_data: *mut c_void);
    #[no_mangle]
    extern "C" fn frame_userdata_cb(user_data: *mut c_void);
    #[no_mangle]
    extern "C" fn cleanup_userdata_cb(user_data: *mut c_void);
    #[no_mangle]
    extern "C" fn event_userdata_cb(event: *const ffi::sapp_event, user_data: *mut c_void);
    #[no_mangle]
    extern "C" fn fail_userdata_cb(message: *const c_char, user_data: *mut c_void);
    // #[no_mangle]
    // extern "C" fn stream_userdata_cb(
    //     buffer: *mut f32,
    //     num_frames: c_int,
    //     num_channels: c_int,
    //     user_data: *mut c_void,
    // );
}

// Why `#[no_mangle]` for C callback functions? I'm not sure, but the nomicon has some note:
// https://doc.rust-lang.org/nomicon/ffi.html#calling-rust-code-from-c
// Also, you might be interested in "name mangling". I would google about it.

impl<T: RApp> RAppFfiCallback for T {
    #[no_mangle]
    extern "C" fn init_userdata_cb(user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.init();
    }

    #[no_mangle]
    extern "C" fn frame_userdata_cb(user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.frame();
    }

    #[no_mangle]
    extern "C" fn cleanup_userdata_cb(user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.cleanup();
    }

    #[no_mangle]
    extern "C" fn event_userdata_cb(event: *const ffi::sapp_event, user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        // note that `RAppEvent` is just an alias of `sapp_event`
        let ev: &Event = unsafe { &*(event as *const _) };

        me.event(ev);
    }

    #[no_mangle]
    extern "C" fn fail_userdata_cb(message: *const c_char, user_data: *mut c_void) {
        let msg = unsafe { CStr::from_ptr(message) };

        let msg = match msg.to_str() {
            Ok(msg) => msg,
            Err(err) => {
                eprintln!("Failed to read sokol_app message: {}", err);
                return;
            }
        };

        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.fail(msg);
    }

    // #[no_mangle]
    // extern "C" fn stream_userdata_cb(
    //     buffer: *mut f32,
    //     num_frames: c_int,
    //     num_channels: c_int,
    //     user_data: *mut c_void,
    // ) {
    //     let arr = unsafe {
    //         let n_bytes = num_frames * num_channels;
    //         std::slice::from_raw_parts_mut(buffer, n_bytes as usize)
    //     };
    //
    //     let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
    //     me.audio_stream(arr, num_frames, num_channels);
    // }
}

// --------------------------------------------------------------------------------
// enums

/// Type of [`rokol::app::Event`]
///
/// [`rokol::app::Event`]: crate::app::Event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum EventType {
    InvalidEvent = ffi::sapp_event_type_SAPP_EVENTTYPE_INVALID as u32,
    KeyDown = ffi::sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN as u32,
    KeyUp = ffi::sapp_event_type_SAPP_EVENTTYPE_KEY_UP as u32,
    Char = ffi::sapp_event_type_SAPP_EVENTTYPE_CHAR as u32,
    MouseDown = ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN as u32,
    MouseUp = ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP as u32,
    MouseScroll = ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL as u32,
    MouseMove = ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE as u32,
    MouseEnter = ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER as u32,
    MouseLeave = ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE as u32,
    /// (Multi touch)
    TouchesBegin = ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN as u32,
    /// (Multi touch)
    TouchesMoved = ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED as u32,
    /// (Multi touch)
    TouchesEnded = ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED as u32,
    /// (Multi touch)
    TouchesCancelled = ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED as u32,
    Resized = ffi::sapp_event_type_SAPP_EVENTTYPE_RESIZED as u32,
    Iconified = ffi::sapp_event_type_SAPP_EVENTTYPE_ICONIFIED as u32,
    Restored = ffi::sapp_event_type_SAPP_EVENTTYPE_RESTORED as u32,
    /// (Mobile)
    Suspended = ffi::sapp_event_type_SAPP_EVENTTYPE_SUSPENDED as u32,
    /// (Mobile)
    Resumed = ffi::sapp_event_type_SAPP_EVENTTYPE_RESUMED as u32,
    UpdateCursor = ffi::sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR as u32,
    QuitRequested = ffi::sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED as u32,
    ClipboardPasted = ffi::sapp_event_type_SAPP_EVENTTYPE_CLIPBOARD_PASTED as u32,
    FilesDropped = ffi::sapp_event_type_SAPP_EVENTTYPE_FILES_DROPPED as u32,
    _Num = ffi::sapp_event_type__SAPP_EVENTTYPE_NUM as u32,
    _ForceU32 = ffi::sapp_event_type__SAPP_EVENTTYPE_FORCE_U32 as u32,
}

impl EventType {
    pub fn from_u32(x: u32) -> Option<Self> {
        Some(match x {
            ffi::sapp_event_type_SAPP_EVENTTYPE_INVALID => Self::InvalidEvent,
            ffi::sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN => Self::KeyDown,
            ffi::sapp_event_type_SAPP_EVENTTYPE_KEY_UP => Self::KeyUp,
            ffi::sapp_event_type_SAPP_EVENTTYPE_CHAR => Self::Char,
            ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN => Self::MouseDown,
            ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP => Self::MouseUp,
            ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL => Self::MouseScroll,
            ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE => Self::MouseMove,
            ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER => Self::MouseEnter,
            ffi::sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE => Self::MouseLeave,
            ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN => Self::TouchesBegin,
            ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED => Self::TouchesMoved,
            ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED => Self::TouchesEnded,
            ffi::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED => Self::TouchesCancelled,
            ffi::sapp_event_type_SAPP_EVENTTYPE_RESIZED => Self::Resized,
            ffi::sapp_event_type_SAPP_EVENTTYPE_ICONIFIED => Self::Iconified,
            ffi::sapp_event_type_SAPP_EVENTTYPE_RESTORED => Self::Restored,
            ffi::sapp_event_type_SAPP_EVENTTYPE_SUSPENDED => Self::Suspended,
            ffi::sapp_event_type_SAPP_EVENTTYPE_RESUMED => Self::Resumed,
            ffi::sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR => Self::UpdateCursor,
            ffi::sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED => Self::QuitRequested,
            ffi::sapp_event_type_SAPP_EVENTTYPE_CLIPBOARD_PASTED => Self::ClipboardPasted,
            ffi::sapp_event_type_SAPP_EVENTTYPE_FILES_DROPPED => Self::FilesDropped,
            // ffi::sapp_event_type__SAPP_EVENTTYPE_NUM
            // ffi::sapp_event_type__SAPP_EVENTTYPE_FORCE_U32
            _ => return None,
        })
    }
}

/// [`rokol::app`] keycode
///
/// [`rokol::app`]: crate::app
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Key {
    InvalidKey = ffi::sapp_keycode_SAPP_KEYCODE_INVALID as u32,
    Space = ffi::sapp_keycode_SAPP_KEYCODE_SPACE as u32,
    Apostrophe = ffi::sapp_keycode_SAPP_KEYCODE_APOSTROPHE as u32,
    Comma = ffi::sapp_keycode_SAPP_KEYCODE_COMMA as u32,
    Minus = ffi::sapp_keycode_SAPP_KEYCODE_MINUS as u32,
    Period = ffi::sapp_keycode_SAPP_KEYCODE_PERIOD as u32,
    Slash = ffi::sapp_keycode_SAPP_KEYCODE_SLASH as u32,
    Kbd0 = ffi::sapp_keycode_SAPP_KEYCODE_0 as u32,
    Kbd1 = ffi::sapp_keycode_SAPP_KEYCODE_1 as u32,
    Kbd2 = ffi::sapp_keycode_SAPP_KEYCODE_2 as u32,
    Kbd3 = ffi::sapp_keycode_SAPP_KEYCODE_3 as u32,
    Kbd4 = ffi::sapp_keycode_SAPP_KEYCODE_4 as u32,
    Kbd5 = ffi::sapp_keycode_SAPP_KEYCODE_5 as u32,
    Kbd6 = ffi::sapp_keycode_SAPP_KEYCODE_6 as u32,
    Kbd7 = ffi::sapp_keycode_SAPP_KEYCODE_7 as u32,
    Kbd8 = ffi::sapp_keycode_SAPP_KEYCODE_8 as u32,
    Kbd9 = ffi::sapp_keycode_SAPP_KEYCODE_9 as u32,
    Semicolon = ffi::sapp_keycode_SAPP_KEYCODE_SEMICOLON as u32,
    Equal = ffi::sapp_keycode_SAPP_KEYCODE_EQUAL as u32,
    A = ffi::sapp_keycode_SAPP_KEYCODE_A as u32,
    B = ffi::sapp_keycode_SAPP_KEYCODE_B as u32,
    C = ffi::sapp_keycode_SAPP_KEYCODE_C as u32,
    D = ffi::sapp_keycode_SAPP_KEYCODE_D as u32,
    E = ffi::sapp_keycode_SAPP_KEYCODE_E as u32,
    F = ffi::sapp_keycode_SAPP_KEYCODE_F as u32,
    G = ffi::sapp_keycode_SAPP_KEYCODE_G as u32,
    H = ffi::sapp_keycode_SAPP_KEYCODE_H as u32,
    I = ffi::sapp_keycode_SAPP_KEYCODE_I as u32,
    J = ffi::sapp_keycode_SAPP_KEYCODE_J as u32,
    K = ffi::sapp_keycode_SAPP_KEYCODE_K as u32,
    L = ffi::sapp_keycode_SAPP_KEYCODE_L as u32,
    M = ffi::sapp_keycode_SAPP_KEYCODE_M as u32,
    N = ffi::sapp_keycode_SAPP_KEYCODE_N as u32,
    O = ffi::sapp_keycode_SAPP_KEYCODE_O as u32,
    P = ffi::sapp_keycode_SAPP_KEYCODE_P as u32,
    Q = ffi::sapp_keycode_SAPP_KEYCODE_Q as u32,
    R = ffi::sapp_keycode_SAPP_KEYCODE_R as u32,
    S = ffi::sapp_keycode_SAPP_KEYCODE_S as u32,
    T = ffi::sapp_keycode_SAPP_KEYCODE_T as u32,
    U = ffi::sapp_keycode_SAPP_KEYCODE_U as u32,
    V = ffi::sapp_keycode_SAPP_KEYCODE_V as u32,
    W = ffi::sapp_keycode_SAPP_KEYCODE_W as u32,
    X = ffi::sapp_keycode_SAPP_KEYCODE_X as u32,
    Y = ffi::sapp_keycode_SAPP_KEYCODE_Y as u32,
    Z = ffi::sapp_keycode_SAPP_KEYCODE_Z as u32,
    LeftBracket = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET as u32,
    Backslash = ffi::sapp_keycode_SAPP_KEYCODE_BACKSLASH as u32,
    RightBracket = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET as u32,
    GraveAccent = ffi::sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT as u32,
    World1 = ffi::sapp_keycode_SAPP_KEYCODE_WORLD_1 as u32,
    World2 = ffi::sapp_keycode_SAPP_KEYCODE_WORLD_2 as u32,
    Escape = ffi::sapp_keycode_SAPP_KEYCODE_ESCAPE as u32,
    Enter = ffi::sapp_keycode_SAPP_KEYCODE_ENTER as u32,
    Tab = ffi::sapp_keycode_SAPP_KEYCODE_TAB as u32,
    Backspace = ffi::sapp_keycode_SAPP_KEYCODE_BACKSPACE as u32,
    Insert = ffi::sapp_keycode_SAPP_KEYCODE_INSERT as u32,
    Delete = ffi::sapp_keycode_SAPP_KEYCODE_DELETE as u32,
    Right = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT as u32,
    Left = ffi::sapp_keycode_SAPP_KEYCODE_LEFT as u32,
    Down = ffi::sapp_keycode_SAPP_KEYCODE_DOWN as u32,
    Up = ffi::sapp_keycode_SAPP_KEYCODE_UP as u32,
    PageUp = ffi::sapp_keycode_SAPP_KEYCODE_PAGE_UP as u32,
    PageDown = ffi::sapp_keycode_SAPP_KEYCODE_PAGE_DOWN as u32,
    Home = ffi::sapp_keycode_SAPP_KEYCODE_HOME as u32,
    End = ffi::sapp_keycode_SAPP_KEYCODE_END as u32,
    CapsLock = ffi::sapp_keycode_SAPP_KEYCODE_CAPS_LOCK as u32,
    ScrollLock = ffi::sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK as u32,
    NumLock = ffi::sapp_keycode_SAPP_KEYCODE_NUM_LOCK as u32,
    PrintScreen = ffi::sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN as u32,
    Pause = ffi::sapp_keycode_SAPP_KEYCODE_PAUSE as u32,
    F1 = ffi::sapp_keycode_SAPP_KEYCODE_F1 as u32,
    F2 = ffi::sapp_keycode_SAPP_KEYCODE_F2 as u32,
    F3 = ffi::sapp_keycode_SAPP_KEYCODE_F3 as u32,
    F4 = ffi::sapp_keycode_SAPP_KEYCODE_F4 as u32,
    F5 = ffi::sapp_keycode_SAPP_KEYCODE_F5 as u32,
    F6 = ffi::sapp_keycode_SAPP_KEYCODE_F6 as u32,
    F7 = ffi::sapp_keycode_SAPP_KEYCODE_F7 as u32,
    F8 = ffi::sapp_keycode_SAPP_KEYCODE_F8 as u32,
    F9 = ffi::sapp_keycode_SAPP_KEYCODE_F9 as u32,
    F10 = ffi::sapp_keycode_SAPP_KEYCODE_F10 as u32,
    F11 = ffi::sapp_keycode_SAPP_KEYCODE_F11 as u32,
    F12 = ffi::sapp_keycode_SAPP_KEYCODE_F12 as u32,
    F13 = ffi::sapp_keycode_SAPP_KEYCODE_F13 as u32,
    F14 = ffi::sapp_keycode_SAPP_KEYCODE_F14 as u32,
    F15 = ffi::sapp_keycode_SAPP_KEYCODE_F15 as u32,
    F16 = ffi::sapp_keycode_SAPP_KEYCODE_F16 as u32,
    F17 = ffi::sapp_keycode_SAPP_KEYCODE_F17 as u32,
    F18 = ffi::sapp_keycode_SAPP_KEYCODE_F18 as u32,
    F19 = ffi::sapp_keycode_SAPP_KEYCODE_F19 as u32,
    F20 = ffi::sapp_keycode_SAPP_KEYCODE_F20 as u32,
    F21 = ffi::sapp_keycode_SAPP_KEYCODE_F21 as u32,
    F22 = ffi::sapp_keycode_SAPP_KEYCODE_F22 as u32,
    F23 = ffi::sapp_keycode_SAPP_KEYCODE_F23 as u32,
    F24 = ffi::sapp_keycode_SAPP_KEYCODE_F24 as u32,
    F25 = ffi::sapp_keycode_SAPP_KEYCODE_F25 as u32,
    KP0 = ffi::sapp_keycode_SAPP_KEYCODE_KP_0 as u32,
    KP1 = ffi::sapp_keycode_SAPP_KEYCODE_KP_1 as u32,
    KP2 = ffi::sapp_keycode_SAPP_KEYCODE_KP_2 as u32,
    KP3 = ffi::sapp_keycode_SAPP_KEYCODE_KP_3 as u32,
    KP4 = ffi::sapp_keycode_SAPP_KEYCODE_KP_4 as u32,
    KP5 = ffi::sapp_keycode_SAPP_KEYCODE_KP_5 as u32,
    KP6 = ffi::sapp_keycode_SAPP_KEYCODE_KP_6 as u32,
    KP7 = ffi::sapp_keycode_SAPP_KEYCODE_KP_7 as u32,
    KP8 = ffi::sapp_keycode_SAPP_KEYCODE_KP_8 as u32,
    KP9 = ffi::sapp_keycode_SAPP_KEYCODE_KP_9 as u32,
    KPDecimal = ffi::sapp_keycode_SAPP_KEYCODE_KP_DECIMAL as u32,
    KPDivide = ffi::sapp_keycode_SAPP_KEYCODE_KP_DIVIDE as u32,
    KPMultiply = ffi::sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY as u32,
    KPSubtract = ffi::sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT as u32,
    KPAdd = ffi::sapp_keycode_SAPP_KEYCODE_KP_ADD as u32,
    KPEnter = ffi::sapp_keycode_SAPP_KEYCODE_KP_ENTER as u32,
    KPEqual = ffi::sapp_keycode_SAPP_KEYCODE_KP_EQUAL as u32,
    LeftShift = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT as u32,
    LeftControl = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL as u32,
    LeftAlt = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_ALT as u32,
    LeftSuper = ffi::sapp_keycode_SAPP_KEYCODE_LEFT_SUPER as u32,
    RightShift = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT as u32,
    RightControl = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL as u32,
    RightAlt = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_ALT as u32,
    RightSuper = ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER as u32,
    Menu = ffi::sapp_keycode_SAPP_KEYCODE_MENU as u32,
}

impl Key {
    pub fn from_u32(x: u32) -> Option<Self> {
        Some(match x {
            ffi::sapp_keycode_SAPP_KEYCODE_INVALID => Self::InvalidKey,
            ffi::sapp_keycode_SAPP_KEYCODE_SPACE => Self::Space,
            ffi::sapp_keycode_SAPP_KEYCODE_APOSTROPHE => Self::Apostrophe,
            ffi::sapp_keycode_SAPP_KEYCODE_COMMA => Self::Comma,
            ffi::sapp_keycode_SAPP_KEYCODE_MINUS => Self::Minus,
            ffi::sapp_keycode_SAPP_KEYCODE_PERIOD => Self::Period,
            ffi::sapp_keycode_SAPP_KEYCODE_SLASH => Self::Slash,
            ffi::sapp_keycode_SAPP_KEYCODE_0 => Self::Kbd0,
            ffi::sapp_keycode_SAPP_KEYCODE_1 => Self::Kbd1,
            ffi::sapp_keycode_SAPP_KEYCODE_2 => Self::Kbd2,
            ffi::sapp_keycode_SAPP_KEYCODE_3 => Self::Kbd3,
            ffi::sapp_keycode_SAPP_KEYCODE_4 => Self::Kbd4,
            ffi::sapp_keycode_SAPP_KEYCODE_5 => Self::Kbd5,
            ffi::sapp_keycode_SAPP_KEYCODE_6 => Self::Kbd6,
            ffi::sapp_keycode_SAPP_KEYCODE_7 => Self::Kbd7,
            ffi::sapp_keycode_SAPP_KEYCODE_8 => Self::Kbd8,
            ffi::sapp_keycode_SAPP_KEYCODE_9 => Self::Kbd9,
            ffi::sapp_keycode_SAPP_KEYCODE_SEMICOLON => Self::Semicolon,
            ffi::sapp_keycode_SAPP_KEYCODE_EQUAL => Self::Equal,
            ffi::sapp_keycode_SAPP_KEYCODE_A => Self::A,
            ffi::sapp_keycode_SAPP_KEYCODE_B => Self::B,
            ffi::sapp_keycode_SAPP_KEYCODE_C => Self::C,
            ffi::sapp_keycode_SAPP_KEYCODE_D => Self::D,
            ffi::sapp_keycode_SAPP_KEYCODE_E => Self::E,
            ffi::sapp_keycode_SAPP_KEYCODE_F => Self::F,
            ffi::sapp_keycode_SAPP_KEYCODE_G => Self::G,
            ffi::sapp_keycode_SAPP_KEYCODE_H => Self::H,
            ffi::sapp_keycode_SAPP_KEYCODE_I => Self::I,
            ffi::sapp_keycode_SAPP_KEYCODE_J => Self::J,
            ffi::sapp_keycode_SAPP_KEYCODE_K => Self::K,
            ffi::sapp_keycode_SAPP_KEYCODE_L => Self::L,
            ffi::sapp_keycode_SAPP_KEYCODE_M => Self::M,
            ffi::sapp_keycode_SAPP_KEYCODE_N => Self::N,
            ffi::sapp_keycode_SAPP_KEYCODE_O => Self::O,
            ffi::sapp_keycode_SAPP_KEYCODE_P => Self::P,
            ffi::sapp_keycode_SAPP_KEYCODE_Q => Self::Q,
            ffi::sapp_keycode_SAPP_KEYCODE_R => Self::R,
            ffi::sapp_keycode_SAPP_KEYCODE_S => Self::S,
            ffi::sapp_keycode_SAPP_KEYCODE_T => Self::T,
            ffi::sapp_keycode_SAPP_KEYCODE_U => Self::U,
            ffi::sapp_keycode_SAPP_KEYCODE_V => Self::V,
            ffi::sapp_keycode_SAPP_KEYCODE_W => Self::W,
            ffi::sapp_keycode_SAPP_KEYCODE_X => Self::X,
            ffi::sapp_keycode_SAPP_KEYCODE_Y => Self::Y,
            ffi::sapp_keycode_SAPP_KEYCODE_Z => Self::Z,
            ffi::sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET => Self::LeftBracket,
            ffi::sapp_keycode_SAPP_KEYCODE_BACKSLASH => Self::Backslash,
            ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET => Self::RightBracket,
            ffi::sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT => Self::GraveAccent,
            ffi::sapp_keycode_SAPP_KEYCODE_WORLD_1 => Self::World1,
            ffi::sapp_keycode_SAPP_KEYCODE_WORLD_2 => Self::World2,
            ffi::sapp_keycode_SAPP_KEYCODE_ESCAPE => Self::Escape,
            ffi::sapp_keycode_SAPP_KEYCODE_ENTER => Self::Enter,
            ffi::sapp_keycode_SAPP_KEYCODE_TAB => Self::Tab,
            ffi::sapp_keycode_SAPP_KEYCODE_BACKSPACE => Self::Backspace,
            ffi::sapp_keycode_SAPP_KEYCODE_INSERT => Self::Insert,
            ffi::sapp_keycode_SAPP_KEYCODE_DELETE => Self::Delete,
            ffi::sapp_keycode_SAPP_KEYCODE_RIGHT => Self::Right,
            ffi::sapp_keycode_SAPP_KEYCODE_LEFT => Self::Left,
            ffi::sapp_keycode_SAPP_KEYCODE_DOWN => Self::Down,
            ffi::sapp_keycode_SAPP_KEYCODE_UP => Self::Up,
            ffi::sapp_keycode_SAPP_KEYCODE_PAGE_UP => Self::PageUp,
            ffi::sapp_keycode_SAPP_KEYCODE_PAGE_DOWN => Self::PageDown,
            ffi::sapp_keycode_SAPP_KEYCODE_HOME => Self::Home,
            ffi::sapp_keycode_SAPP_KEYCODE_END => Self::End,
            ffi::sapp_keycode_SAPP_KEYCODE_CAPS_LOCK => Self::CapsLock,
            ffi::sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK => Self::ScrollLock,
            ffi::sapp_keycode_SAPP_KEYCODE_NUM_LOCK => Self::NumLock,
            ffi::sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN => Self::PrintScreen,
            ffi::sapp_keycode_SAPP_KEYCODE_PAUSE => Self::Pause,
            ffi::sapp_keycode_SAPP_KEYCODE_F1 => Self::F1,
            ffi::sapp_keycode_SAPP_KEYCODE_F2 => Self::F2,
            ffi::sapp_keycode_SAPP_KEYCODE_F3 => Self::F3,
            ffi::sapp_keycode_SAPP_KEYCODE_F4 => Self::F4,
            ffi::sapp_keycode_SAPP_KEYCODE_F5 => Self::F5,
            ffi::sapp_keycode_SAPP_KEYCODE_F6 => Self::F6,
            ffi::sapp_keycode_SAPP_KEYCODE_F7 => Self::F7,
            ffi::sapp_keycode_SAPP_KEYCODE_F8 => Self::F8,
            ffi::sapp_keycode_SAPP_KEYCODE_F9 => Self::F9,
            ffi::sapp_keycode_SAPP_KEYCODE_F10 => Self::F10,
            ffi::sapp_keycode_SAPP_KEYCODE_F11 => Self::F11,
            ffi::sapp_keycode_SAPP_KEYCODE_F12 => Self::F12,
            ffi::sapp_keycode_SAPP_KEYCODE_F13 => Self::F13,
            ffi::sapp_keycode_SAPP_KEYCODE_F14 => Self::F14,
            ffi::sapp_keycode_SAPP_KEYCODE_F15 => Self::F15,
            ffi::sapp_keycode_SAPP_KEYCODE_F16 => Self::F16,
            ffi::sapp_keycode_SAPP_KEYCODE_F17 => Self::F17,
            ffi::sapp_keycode_SAPP_KEYCODE_F18 => Self::F18,
            ffi::sapp_keycode_SAPP_KEYCODE_F19 => Self::F19,
            ffi::sapp_keycode_SAPP_KEYCODE_F20 => Self::F20,
            ffi::sapp_keycode_SAPP_KEYCODE_F21 => Self::F21,
            ffi::sapp_keycode_SAPP_KEYCODE_F22 => Self::F22,
            ffi::sapp_keycode_SAPP_KEYCODE_F23 => Self::F23,
            ffi::sapp_keycode_SAPP_KEYCODE_F24 => Self::F24,
            ffi::sapp_keycode_SAPP_KEYCODE_F25 => Self::F25,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_0 => Self::KP0,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_1 => Self::KP1,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_2 => Self::KP2,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_3 => Self::KP3,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_4 => Self::KP4,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_5 => Self::KP5,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_6 => Self::KP6,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_7 => Self::KP7,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_8 => Self::KP8,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_9 => Self::KP9,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_DECIMAL => Self::KPDecimal,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_DIVIDE => Self::KPDivide,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY => Self::KPMultiply,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT => Self::KPSubtract,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_ADD => Self::KPAdd,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_ENTER => Self::KPEnter,
            ffi::sapp_keycode_SAPP_KEYCODE_KP_EQUAL => Self::KPEqual,
            ffi::sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT => Self::LeftShift,
            ffi::sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL => Self::LeftControl,
            ffi::sapp_keycode_SAPP_KEYCODE_LEFT_ALT => Self::LeftAlt,
            ffi::sapp_keycode_SAPP_KEYCODE_LEFT_SUPER => Self::LeftSuper,
            ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT => Self::RightShift,
            ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL => Self::RightControl,
            ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_ALT => Self::RightAlt,
            ffi::sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER => Self::RightSuper,
            ffi::sapp_keycode_SAPP_KEYCODE_MENU => Self::Menu,
            _ => return None,
        })
    }
}

/// [`rokol::app`] mouse input
///
/// [`rokol::app`]: crate::app
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Mouse {
    Invalid = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID as u32,
    Left = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT as u32,
    Right = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT as u32,
    Middle = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE as u32,
}

impl Mouse {
    pub fn from_u32(x: u32) -> Option<Self> {
        Some(match x {
            ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID => Self::Invalid,
            ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT => Self::Left,
            ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT => Self::Right,
            ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE => Self::Middle,
            _ => return None,
        })
    }
}

bitflags! {
    /// Rokol modifier keys as bitflags
    #[repr(C)]
    pub struct Mod: u32 {
        const SHIFT = ffi::SAPP_MODIFIER_SHIFT as u32;
        const CONTROL = ffi::SAPP_MODIFIER_CTRL as u32;
        const ALT = ffi::SAPP_MODIFIER_ALT as u32;
        const SUPER = ffi::SAPP_MODIFIER_SUPER as u32;
    }
}

// --------------------------------------------------------------------------------
// Re-exports
//
// `Debug` and `Default` are implemented by `bindgen`. If we need more, we add methods in the FFI
// module.

/// [`rokol::app`] touch input
///
/// [`rokol::app`]: crate::app
pub type TouchPoint = ffi::sapp_touchpoint;

/// [`rokol::app`] application event
///
/// [`rokol::app`]: crate::app
pub type Event = ffi::sapp_event;

pub type Range = ffi::sapp_range;

pub type IconDesc = ffi::sapp_icon_desc;

pub fn set_icon(icon: &IconDesc) {
    unsafe {
        ffi::sapp_set_icon(icon as *const _);
    }
}

/// Returns true after Rokol app is initialized
pub fn is_valid() -> bool {
    unsafe { ffi::sapp_isvalid() }
}

/// Width of the current frame buffer in pixels
///
/// It returns actual pixels, not scaled size (e.g. 2560x1440 on macbook pro 15.6 inch, where scaled
/// screen size is 1280x720). Divided it with [`dpi_scale`] to get size for e.g.
/// [`crate::gfx::ImageDesc`].
pub fn width() -> u32 {
    // it's always bigger than zero, so this is safe
    unsafe { ffi::sapp_width() as u32 }
}

/// Height of the current frame buffer in pixels
///
/// screen size is 1280x720). Divided it with [`dpi_scale`] to get size for e.g.
/// [`crate::gfx::ImageDesc`].
pub fn height() -> u32 {
    // it's always bigger than zero, so this is safe
    unsafe { ffi::sapp_height() as u32 }
}

/// (Non-Sokol) size of the current frame buffer in pixels
///
/// This function is Rokol-only and Sokol doesn't have a corresponding function.
pub fn size() -> [u32; 2] {
    [self::width(), self::height()]
}

pub fn width_f() -> f32 {
    unsafe { ffi::sapp_widthf() }
}

pub fn height_f() -> f32 {
    unsafe { ffi::sapp_heightf() }
}

/// (Non-Sokol) size of the current frame buffer in pixels
pub fn size_f() -> [f32; 2] {
    [self::width_f(), self::height_f()]
}

/// (Non-Sokol) size of the window (not the frame buffer)
pub fn size_f_scaled() -> [f32; 2] {
    [
        self::width_f() / self::dpi_scale(),
        self::height_f() / self::dpi_scale(),
    ]
}

/// TODO: use [`crate::gfx::PixelFormat`] as return value
pub fn color_fmt() -> i32 {
    unsafe { ffi::sapp_color_format() }
}

/// TODO: use [`crate::gfx::PixelFormat`]
pub fn depth_format() -> i32 {
    unsafe { ffi::sapp_depth_format() }
}

/// Default frame buffer count
pub fn sample_count() -> u32 {
    unsafe { ffi::sapp_sample_count() as u32 }
}

/// True when high_dpi was requested and actually running in a high-dpi scenario
pub fn is_high_dpi() -> bool {
    unsafe { ffi::sapp_high_dpi() }
}

/// Dpi scaling factor (window pixels to framebuffer pixels)
pub fn dpi_scale() -> f32 {
    unsafe { ffi::sapp_dpi_scale() }
}

/// (Mobile) Show or hide the mobile device onscreen keyboard
pub fn set_show_kbd(do_show: bool) {
    unsafe {
        ffi::sapp_show_keyboard(do_show);
    }
}

/// (Mobile) True if the mobile device onscreen keyboard is currently shown
pub fn is_kbd_shown() -> bool {
    unsafe { ffi::sapp_keyboard_shown() }
}

pub fn is_fullscreen() -> bool {
    unsafe { ffi::sapp_is_fullscreen() }
}

pub fn toggle_fullscreen() {
    unsafe { ffi::sapp_toggle_fullscreen() }
}

/// Show or hide the mouse cursor
pub fn set_show_mouse(show: bool) {
    unsafe {
        ffi::sapp_show_mouse(show);
    }
}

/// Show or hide the mouse cursor
pub fn is_mouse_shown() -> bool {
    unsafe { ffi::sapp_mouse_shown() }
}
//
// Enable/disable mouse-pointer-lock mode
pub fn set_lock_mouse(lock: bool) {
    unsafe {
        ffi::sapp_lock_mouse(lock);
    }
}

/// True if in mouse-pointer-lock mode (this may toggle a few frames later)
pub fn is_mouse_locked() -> bool {
    unsafe { ffi::sapp_mouse_locked() }
}

// /// The userdata pointer provided in [`RAppDesc`]
// pub fn userdata() -> *mut c_void {
//     unsafe { ffi::sapp_userdata() }
// }

// /// Copy of the sapp_desc structure
// pub fn query_desc() -> RAppDesc {
//     unsafe { ffi::sapp_query_desc() }
// }

// --------------------------------------------------------------------------------
// Quit protocol

/// Initiate a "soft quit" (sends `SAPP_EVENTTYPE_QUIT_REQUESTED`)
pub fn request_quit() {
    unsafe { ffi::sapp_request_quit() }
}

/// Cancel a pending quit (when `SAPP_EVENTTYPE_QUIT_REQUESTED` has been received)
pub fn cancel_quit() {
    unsafe { ffi::sapp_cancel_quit() }
}

/// Initiate a "hard quit" (quit application without sending `SAPP_EVENTTYPE_QUIT_REQUSTED`)
pub fn quit() {
    unsafe {
        ffi::sapp_quit();
    }
}

/// Call from inside event callback to consume the current event (don't forward to platform)
///
/// NOTE: this function is only implemented for HTML5 backend (see `sokol_app.h` ln 352)
pub fn consume_event() {
    unsafe {
        ffi::sapp_consume_event();
    }
}

/// Current frame counter (for comparison with sapp_event.frame_count)
pub fn frame_count() -> u64 {
    unsafe { ffi::sapp_frame_count() }
}

/// (Clipboard) Write string into clipboard
pub fn set_clipboard(s: &str) -> Result<(), std::ffi::NulError> {
    // unfortunate cost to make it null-terminated
    let c_str = CString::new(s)?;
    unsafe {
        ffi::sapp_set_clipboard_string(c_str.as_ptr() as *mut _);
    }
    Ok(())
}

/// (Clipboard) Read string from clipboard (usually during `SAPP_EVENTTYPE_CLIPBOARD_PASTED`)
pub fn clipboard() -> Result<String, std::str::Utf8Error> {
    let ptr = unsafe { ffi::sapp_get_clipboard_string() };
    let c_str = unsafe { CStr::from_ptr(ptr) };
    c_str.to_str().map(|s| s.to_string())
}

/// (Desktop) Set the window title (only on desktop platforms)
pub fn set_win_title(title: &str) -> Result<(), std::ffi::NulError> {
    let c_str = CString::new(title)?;
    unsafe { ffi::sapp_set_window_title(c_str.as_ptr() as *mut _) };
    Ok(())
}

/// (Drag) The total number of dropped files (after an `SAPP_EVENTTYPE_FILES_DROPPED` event)
pub fn n_dropped_files() -> u32 {
    unsafe { ffi::sapp_get_num_dropped_files() as u32 }
}

/// (Drag) The dropped file paths
pub fn dropped_file_path(ix: u32) -> Result<String, std::str::Utf8Error> {
    let ptr = unsafe { ffi::sapp_get_dropped_file_path(ix as i32) };
    let c_str = unsafe { CString::from_raw(ptr as *mut _) };
    c_str.to_str().map(|s| s.to_string())
}

// --------------------------------------------------------------------------------
// Platform-dependent API

// // /* GL: return true when GLES2 fallback is active (to detect fallback from GLES3) */
// // SOKOL_API_DECL bool sapp_gles2(void);

// // /* HTML5: enable or disable the hardwired "Leave Site?" dialog box */
// // SOKOL_API_DECL void sapp_html5_ask_leave_site(bool ask);
// // /* HTML5: get byte size of a dropped file */
// // SOKOL_API_DECL uint32_t sapp_html5_get_dropped_file_size(int index);
// // /* HTML5: asynchronously load the content of a dropped file */
// // SOKOL_API_DECL void sapp_html5_fetch_dropped_file(const sapp_html5_fetch_request* request);

// // /* Metal: get bridged pointer to Metal device object */
// // SOKOL_API_DECL const void* sapp_metal_get_device(void);
// // /* Metal: get bridged pointer to this frame's renderpass descriptor */
// // SOKOL_API_DECL const void* sapp_metal_get_renderpass_descriptor(void);
// // /* Metal: get bridged pointer to current drawable */
// // SOKOL_API_DECL const void* sapp_metal_get_drawable(void);
// // /* macOS: get bridged pointer to macOS NSWindow */
// // SOKOL_API_DECL const void* sapp_macos_get_window(void);
// // /* iOS: get bridged pointer to iOS UIWindow */
// // SOKOL_API_DECL const void* sapp_ios_get_window(void);

// // /* D3D11: get pointer to ID3D11Device object */
// // SOKOL_API_DECL const void* sapp_d3d11_get_device(void);
// // /* D3D11: get pointer to ID3D11DeviceContext object */
// // SOKOL_API_DECL const void* sapp_d3d11_get_device_context(void);
// // /* D3D11: get pointer to ID3D11RenderTargetView object */
// // SOKOL_API_DECL const void* sapp_d3d11_get_render_target_view(void);
// // /* D3D11: get pointer to ID3D11DepthStencilView */
// // SOKOL_API_DECL const void* sapp_d3d11_get_depth_stencil_view(void);
// // /* Win32: get the HWND window handle */
// // SOKOL_API_DECL const void* sapp_win32_get_hwnd(void);
// sapp_d3d11_get_swapchain

// // /* WebGPU: get WGPUDevice handle */
// // SOKOL_API_DECL const void* sapp_wgpu_get_device(void);
// // /* WebGPU: get swapchain's WGPUTextureView handle for rendering */
// // SOKOL_API_DECL const void* sapp_wgpu_get_render_view(void);
// // /* WebGPU: get swapchain's MSAA-resolve WGPUTextureView (may return null) */
// // SOKOL_API_DECL const void* sapp_wgpu_get_resolve_view(void);
// // /* WebGPU: get swapchain's WGPUTextureView for the depth-stencil surface */
// // SOKOL_API_DECL const void* sapp_wgpu_get_depth_stencil_view(void);

// // /* Android: get native activity handle */
// // SOKOL_API_DECL const void* sapp_android_get_native_activity(void);
