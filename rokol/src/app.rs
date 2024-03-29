/*!
Application ([`FFI`])

[`FFI`]: rokol_ffi::app

[`Rokol::run`](crate::glue::sapp::Rokol::run) runs an implementation of [`RApp`].
*/

use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};

use {bitflags::bitflags, rokol_ffi::app as ffi};

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
    extern "C" fn init_userdata_cb(user_data: *mut c_void);
    extern "C" fn frame_userdata_cb(user_data: *mut c_void);
    extern "C" fn cleanup_userdata_cb(user_data: *mut c_void);
    extern "C" fn event_userdata_cb(event: *const ffi::sapp_event, user_data: *mut c_void);
    extern "C" fn fail_userdata_cb(message: *const c_char, user_data: *mut c_void);
    // extern "C" fn stream_userdata_cb(
    //     buffer: *mut f32,
    //     num_frames: c_uint,
    //     num_channels: c_uint,
    //     user_data: *mut c_void,
    // );
}

impl<T: RApp> RAppFfiCallback for T {
    extern "C" fn init_userdata_cb(user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.init();
    }

    extern "C" fn frame_userdata_cb(user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.frame();
    }

    extern "C" fn cleanup_userdata_cb(user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        me.cleanup();
    }

    extern "C" fn event_userdata_cb(event: *const ffi::sapp_event, user_data: *mut c_void) {
        let me: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        // note that `RAppEvent` is just an alias of `sapp_event`
        let ev: &Event = unsafe { &*(event as *const _) };

        me.event(ev);
    }

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

    // extern "C" fn stream_userdata_cb(
    //     buffer: *mut f32,
    //     num_frames: c_uint,
    //     num_channels: c_uint,
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

ffi_enum! {
    /// Type of [`rokol::app::Event`]
    ///
    /// [`rokol::app::Event`]: crate::app::Event
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EventType around ffi::sapp_event_type {
        InvalidEvent = SAPP_EVENTTYPE_INVALID,
        KeyDown = SAPP_EVENTTYPE_KEY_DOWN,
        KeyUp = SAPP_EVENTTYPE_KEY_UP,
        Char = SAPP_EVENTTYPE_CHAR,
        MouseDown = SAPP_EVENTTYPE_MOUSE_DOWN,
        MouseUp = SAPP_EVENTTYPE_MOUSE_UP,
        MouseScroll = SAPP_EVENTTYPE_MOUSE_SCROLL,
        MouseMove = SAPP_EVENTTYPE_MOUSE_MOVE,
        MouseEnter = SAPP_EVENTTYPE_MOUSE_ENTER,
        MouseLeave = SAPP_EVENTTYPE_MOUSE_LEAVE,
        /// (Multi touch)
        TouchesBegin = SAPP_EVENTTYPE_TOUCHES_BEGAN,
        /// (Multi touch)
        TouchesMoved = SAPP_EVENTTYPE_TOUCHES_MOVED,
        /// (Multi touch)
        TouchesEnded = SAPP_EVENTTYPE_TOUCHES_ENDED,
        /// (Multi touch)
        TouchesCancelled = SAPP_EVENTTYPE_TOUCHES_CANCELLED,
        Resized = SAPP_EVENTTYPE_RESIZED,
        Iconified = SAPP_EVENTTYPE_ICONIFIED,
        Restored = SAPP_EVENTTYPE_RESTORED,
        Focused = SAPP_EVENTTYPE_FOCUSED,
        Unocused = SAPP_EVENTTYPE_UNFOCUSED,
        /// (Mobile)
        Suspended = SAPP_EVENTTYPE_SUSPENDED,
        /// (Mobile)
        Resumed = SAPP_EVENTTYPE_RESUMED,
        UpdateCursor = SAPP_EVENTTYPE_UPDATE_CURSOR,
        QuitRequested = SAPP_EVENTTYPE_QUIT_REQUESTED,
        ClipboardPasted = SAPP_EVENTTYPE_CLIPBOARD_PASTED,
        FilesDropped = SAPP_EVENTTYPE_FILES_DROPPED,
        _Num = _SAPP_EVENTTYPE_NUM,
        _ForceU32 = _SAPP_EVENTTYPE_FORCE_U32,
    }
}

ffi_enum! {
     /// [`rokol::app`] keycode
     ///
     /// [`rokol::app`]: crate::app
     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
     pub enum Key around ffi::sapp_keycode {
         InvalidKey = SAPP_KEYCODE_INVALID,
         Space = SAPP_KEYCODE_SPACE,
         Apostrophe = SAPP_KEYCODE_APOSTROPHE,
         Comma = SAPP_KEYCODE_COMMA,
         Minus = SAPP_KEYCODE_MINUS,
         Period = SAPP_KEYCODE_PERIOD,
         Slash = SAPP_KEYCODE_SLASH,
         Kbd0 = SAPP_KEYCODE_0,
         Kbd1 = SAPP_KEYCODE_1,
         Kbd2 = SAPP_KEYCODE_2,
         Kbd3 = SAPP_KEYCODE_3,
         Kbd4 = SAPP_KEYCODE_4,
         Kbd5 = SAPP_KEYCODE_5,
         Kbd6 = SAPP_KEYCODE_6,
         Kbd7 = SAPP_KEYCODE_7,
         Kbd8 = SAPP_KEYCODE_8,
         Kbd9 = SAPP_KEYCODE_9,
         Semicolon = SAPP_KEYCODE_SEMICOLON,
         Equal = SAPP_KEYCODE_EQUAL,
         A = SAPP_KEYCODE_A,
         B = SAPP_KEYCODE_B,
         C = SAPP_KEYCODE_C,
         D = SAPP_KEYCODE_D,
         E = SAPP_KEYCODE_E,
         F = SAPP_KEYCODE_F,
         G = SAPP_KEYCODE_G,
         H = SAPP_KEYCODE_H,
         I = SAPP_KEYCODE_I,
         J = SAPP_KEYCODE_J,
         K = SAPP_KEYCODE_K,
         L = SAPP_KEYCODE_L,
         M = SAPP_KEYCODE_M,
         N = SAPP_KEYCODE_N,
         O = SAPP_KEYCODE_O,
         P = SAPP_KEYCODE_P,
         Q = SAPP_KEYCODE_Q,
         R = SAPP_KEYCODE_R,
         S = SAPP_KEYCODE_S,
         T = SAPP_KEYCODE_T,
         U = SAPP_KEYCODE_U,
         V = SAPP_KEYCODE_V,
         W = SAPP_KEYCODE_W,
         X = SAPP_KEYCODE_X,
         Y = SAPP_KEYCODE_Y,
         Z = SAPP_KEYCODE_Z,
         LeftBracket = SAPP_KEYCODE_LEFT_BRACKET,
         Backslash = SAPP_KEYCODE_BACKSLASH,
         RightBracket = SAPP_KEYCODE_RIGHT_BRACKET,
         GraveAccent = SAPP_KEYCODE_GRAVE_ACCENT,
         World1 = SAPP_KEYCODE_WORLD_1,
         World2 = SAPP_KEYCODE_WORLD_2,
         Escape = SAPP_KEYCODE_ESCAPE,
         Enter = SAPP_KEYCODE_ENTER,
         Tab = SAPP_KEYCODE_TAB,
         Backspace = SAPP_KEYCODE_BACKSPACE,
         Insert = SAPP_KEYCODE_INSERT,
         Delete = SAPP_KEYCODE_DELETE,
         Right = SAPP_KEYCODE_RIGHT,
         Left = SAPP_KEYCODE_LEFT,
         Down = SAPP_KEYCODE_DOWN,
         Up = SAPP_KEYCODE_UP,
         PageUp = SAPP_KEYCODE_PAGE_UP,
         PageDown = SAPP_KEYCODE_PAGE_DOWN,
         Home = SAPP_KEYCODE_HOME,
         End = SAPP_KEYCODE_END,
         CapsLock = SAPP_KEYCODE_CAPS_LOCK,
         ScrollLock = SAPP_KEYCODE_SCROLL_LOCK,
         NumLock = SAPP_KEYCODE_NUM_LOCK,
         PrintScreen = SAPP_KEYCODE_PRINT_SCREEN,
         Pause = SAPP_KEYCODE_PAUSE,
         F1 = SAPP_KEYCODE_F1,
         F2 = SAPP_KEYCODE_F2,
         F3 = SAPP_KEYCODE_F3,
         F4 = SAPP_KEYCODE_F4,
         F5 = SAPP_KEYCODE_F5,
         F6 = SAPP_KEYCODE_F6,
         F7 = SAPP_KEYCODE_F7,
         F8 = SAPP_KEYCODE_F8,
         F9 = SAPP_KEYCODE_F9,
         F10 = SAPP_KEYCODE_F10,
         F11 = SAPP_KEYCODE_F11,
         F12 = SAPP_KEYCODE_F12,
         F13 = SAPP_KEYCODE_F13,
         F14 = SAPP_KEYCODE_F14,
         F15 = SAPP_KEYCODE_F15,
         F16 = SAPP_KEYCODE_F16,
         F17 = SAPP_KEYCODE_F17,
         F18 = SAPP_KEYCODE_F18,
         F19 = SAPP_KEYCODE_F19,
         F20 = SAPP_KEYCODE_F20,
         F21 = SAPP_KEYCODE_F21,
         F22 = SAPP_KEYCODE_F22,
         F23 = SAPP_KEYCODE_F23,
         F24 = SAPP_KEYCODE_F24,
         F25 = SAPP_KEYCODE_F25,
         KP0 = SAPP_KEYCODE_KP_0,
         KP1 = SAPP_KEYCODE_KP_1,
         KP2 = SAPP_KEYCODE_KP_2,
         KP3 = SAPP_KEYCODE_KP_3,
         KP4 = SAPP_KEYCODE_KP_4,
         KP5 = SAPP_KEYCODE_KP_5,
         KP6 = SAPP_KEYCODE_KP_6,
         KP7 = SAPP_KEYCODE_KP_7,
         KP8 = SAPP_KEYCODE_KP_8,
         KP9 = SAPP_KEYCODE_KP_9,
         KPDecimal = SAPP_KEYCODE_KP_DECIMAL,
         KPDivide = SAPP_KEYCODE_KP_DIVIDE,
         KPMultiply = SAPP_KEYCODE_KP_MULTIPLY,
         KPSubtract = SAPP_KEYCODE_KP_SUBTRACT,
         KPAdd = SAPP_KEYCODE_KP_ADD,
         KPEnter = SAPP_KEYCODE_KP_ENTER,
         KPEqual = SAPP_KEYCODE_KP_EQUAL,
         LeftShift = SAPP_KEYCODE_LEFT_SHIFT,
         LeftControl = SAPP_KEYCODE_LEFT_CONTROL,
         LeftAlt = SAPP_KEYCODE_LEFT_ALT,
         LeftSuper = SAPP_KEYCODE_LEFT_SUPER,
         RightShift = SAPP_KEYCODE_RIGHT_SHIFT,
         RightControl = SAPP_KEYCODE_RIGHT_CONTROL,
         RightAlt = SAPP_KEYCODE_RIGHT_ALT,
         RightSuper = SAPP_KEYCODE_RIGHT_SUPER,
         Menu = SAPP_KEYCODE_MENU,
     }
}

ffi_enum! {
    /// [`rokol::app`] mouse input
    ///
    /// [`rokol::app`]: crate::app
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Mouse around ffi::sapp_mousebutton {
        Invalid = SAPP_MOUSEBUTTON_INVALID,
        Left = SAPP_MOUSEBUTTON_LEFT,
        Right = SAPP_MOUSEBUTTON_RIGHT,
        Middle = SAPP_MOUSEBUTTON_MIDDLE,
    }
}

bitflags! {
    /// Rokol modifier keys as bitflags
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

/// Frame duration in seconds averaged over a number of frames to smooth out any jittering spikes
pub fn frame_duration() -> f64 {
    unsafe { ffi::sapp_frame_duration() }
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
