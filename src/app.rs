//! Application ([`FFI`])
//!
//! [`FFI`]: rokol_ffi::app

use {
    bitflags::bitflags,
    rokol_ffi::app as ffi,
    std::{
        ffi::{c_void, CStr, CString},
        os::raw::c_char,
    },
};

// --------------------------------------------------------------------------------
// Hidden items from the FFI

// See [`crate::Builder`]

// /// `rokol::app` application description
// pub type RAppDesc = ffi::sapp_desc;

// /// Special run-function for `SOKOL_NO_ENTRY` (in standard mode this is an empty stub)
// pub fn sapp_run(const sapp_desc* desc);

// --------------------------------------------------------------------------------
// Primary trait

/// `rokol::app` callbacks
///
/// All provided function callbacks will be called from the same thread,
/// but this may be different from the thread where `sokol_main()` was called.
pub trait RApp {
    /// Called once after the rendering surface, 3D API and swap chain have been initialized by
    /// `sokol_app`
    fn init(&mut self) {}

    /// Called on the same thread as the init callback, but might be on a different thread than the
    /// `sokol_main()` function
    fn frame(&mut self) {}

    /// Called once after the user quits the application
    ///
    /// Suitable place to implement "really quit?" diaglog.
    ///
    /// The cleanup-callback isn't guaranteed to be called on the web and mobile platforms.
    fn cleanup(&mut self) {
        unsafe {
            rokol_ffi::gfx::sg_shutdown();
        }
    }

    /// Event handling
    ///
    /// Do *not* call any 3D API rendering functions in the event
    /// callback function, since the 3D API context may not be active when the
    /// event callback is called (it may work on some platforms and 3D APIs,
    /// but not others, and the exact behaviour may change between
    /// sokol-app versions).
    fn event(&mut self, _ev: &RAppEvent) {}

    fn fail(&mut self, msg: &str) {
        eprint!("{}", msg);
    }

    // --------------------------------------------------------------------------------
    // C callback functions set to [`RAppDesc`]
}

/// `rokol::app` callbacks for C
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
        let ev: &RAppEvent = unsafe { &*(event as *const _) };

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

/// `rokol::app` event type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
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

/// `rokol::app` keycode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// `rokol::app` mouse input
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mouse {
    Invalid = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
    Left = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
    Right = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT,
    Middle = ffi::sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE,
}

bitflags! {
    /// Rokol modifier keys as bitflags
    #[repr(C)]
    pub struct Mod: u32 {
        const SHIFT = ffi::SAPP_MODIFIER_SHIFT;
        const CONTROL = ffi::SAPP_MODIFIER_CTRL;
        const ALT = ffi::SAPP_MODIFIER_ALT;
        const SUPER = ffi::SAPP_MODIFIER_SUPER;
    }
}

// --------------------------------------------------------------------------------
// Re-exports
//
// `Debug` and `Default` are implemented by `bindgen`. If we need more, we add methods in the FFI
// module.

/// `rokol::app` touch input
pub type TouchPoint = ffi::sapp_touchpoint;

/// `rokol::app` application event
pub type RAppEvent = ffi::sapp_event;

/// Returns true after Rokol app is initialized
pub fn is_valid() -> bool {
    unsafe { ffi::sapp_isvalid() }
}

/// Width of the current frame buffer in pixels
pub fn width() -> u32 {
    // it's always bigger than zero, so this is safe
    unsafe { ffi::sapp_width() as u32 }
}

/// Height of the current frame buffer in pixels
pub fn height() -> u32 {
    // it's always bigger than zero, so this is safe
    unsafe { ffi::sapp_height() as u32 }
}

/// [Non-Sokol] Width and height of the current frame buffer in pixels
///
/// This function is Rokol-only and Sokol doesn't have a corresponding function.
pub fn size() -> [u32; 2] {
    [self::width(), self::height()]
}

/// TODO: use [`crate::gfx::PixelFormat`]
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
