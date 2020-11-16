//! Rokol, wrapper of Sokol libraries
//!
//! Rokol is based on [sokol-rs]. Many thanks to them!
//!
//! [sokol-rs]: https://github.com/code-disaster/sokol-rs

pub use rokol_ffi as ffi;
use std::ffi::CString;

pub mod app;

/// Any error upcasted to [`Box`]
pub type Error = Box<dyn std::error::Error>;

/// Result type with any error upcasted to [`Box`]
pub type Result = std::result::Result<(), Error>;

/// Configuration to run a Rokol application
///
/// Basically a wrapper of [`ffi::app::sapp_desc`].
#[derive(Debug)]
pub struct Rokol {
    /// Preferred width of the window / canvas
    pub w: u32,
    /// Preferred height of the window / canvas
    pub h: u32,
    /// Window title
    pub title: String,

    pub msaa_sample_count: u32,

    /// Preferred swap interval (ignored on some platforms)
    pub swap_interval: u32,
    /// Whether the rendering canvas is full-resolution on HighDPI displays
    pub use_high_dpi: bool,
    /// Whether the window should be created in fullscreen mode
    pub is_full_screen: bool,
    /// Whether the framebuffer should have an alpha channel (ignored on some platforms)
    pub enable_alpha: bool,
    /// If true, user is expected to manage cursor image in `SAPP_EVENTTYPE_UPDATE_CURSOR`
    pub use_user_cursor_image: bool,

    /// Enable clipboard access
    pub enable_clipboard: bool,
    pub max_clipboard_size_in_bytes: u32,

    /// Enable file dropping (drag'n'drop)
    pub enable_drag_and_drop: bool,
    pub n_max_dropped_files: u32,
    /// Max length in bytes of a dropped UTF-8 file path
    pub max_dropped_file_path_len_in_bytes: u32,
    // missing fields from Sokol: html5, ios, gl
}

impl Default for Rokol {
    fn default() -> Self {
        Self {
            w: 640,
            h: 360,
            title: "Untitled".to_string(),
            msaa_sample_count: 1,
            swap_interval: 1,
            use_high_dpi: true,
            is_full_screen: false,
            enable_alpha: true,
            use_user_cursor_image: false,
            enable_clipboard: false,
            max_clipboard_size_in_bytes: 8192,
            enable_drag_and_drop: false,
            n_max_dropped_files: 1,
            max_dropped_file_path_len_in_bytes: 2048,
        }
    }
}

impl Rokol {
    pub fn run<T: app::RApp>(&self, app: &mut T) -> Result {
        let mut desc = ffi::app::sapp_desc::default();

        desc.width = self.w as i32;
        desc.height = self.h as i32;

        let title = CString::new(self.title.as_bytes())?;
        desc.window_title = title.as_ptr() as *mut _;

        desc.swap_interval = self.swap_interval as i32;

        desc.high_dpi = self.use_high_dpi;
        desc.fullscreen = self.is_full_screen;

        desc.alpha = self.enable_alpha;
        desc.user_cursor = self.use_user_cursor_image;

        desc.enable_clipboard = self.enable_clipboard;
        desc.clipboard_size = self.max_clipboard_size_in_bytes as i32;

        desc.enable_dragndrop = self.enable_drag_and_drop;
        desc.max_dropped_files = self.n_max_dropped_files as i32;
        desc.max_dropped_file_path_length = self.max_dropped_file_path_len_in_bytes as i32;

        use self::app::RAppFfiCallback;
        desc.user_data = app as *mut _ as *mut _;

        desc.init_userdata_cb = Some(<T as RAppFfiCallback>::init_userdata_cb);
        desc.frame_userdata_cb = Some(<T as RAppFfiCallback>::frame_userdata_cb);
        desc.cleanup_userdata_cb = Some(<T as RAppFfiCallback>::cleanup_userdata_cb);
        desc.event_userdata_cb = Some(<T as RAppFfiCallback>::event_userdata_cb);
        desc.fail_userdata_cb = Some(<T as RAppFfiCallback>::fail_userdata_cb);

        // desc.stream_userdata_cb = Some(<T as RAppFfiCallback>::stream_userdata_cb);

        unsafe {
            rokol_ffi::app::sapp_run(&mut desc as *mut _);
        }

        Ok(())
    }
}
