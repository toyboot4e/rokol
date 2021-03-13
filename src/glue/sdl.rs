/*!
Glue code for SDL support
*/

#![allow(dead_code)]

pub use rokol_ffi::gfx::sg_context_desc as SgContextDesc;

use crate::gfx as rg;

/// Enum compatible with [`PixelFormat`] in `rokol::gfx`
///
/// [`PixelFormat`]: crate::gfx::PixelFormat
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ColorFormat {
    Rgba8 = rg::PixelFormat::Rgba8 as u32,
    Bgra8 = rg::PixelFormat::Bgra8 as u32,
}

/// Enum compatible with [`PixelFormat`] in `rokol::gfx`
///
/// [`PixelFormat`]: crate::gfx::PixelFormat
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DepthFormat {
    Depth = rg::PixelFormat::Depth as u32,
    DepthStencil = rg::PixelFormat::DepthStencil as u32,
}

#[derive(Debug, Clone)]
pub struct ResourceSettings {
    pub color_format: ColorFormat,
    pub depth_format: DepthFormat,
    /// MSAA sample count of the default frame buffer
    pub sample_count: u32,
}

impl Default for ResourceSettings {
    fn default() -> Self {
        Self {
            color_format: ColorFormat::Rgba8,
            depth_format: DepthFormat::Depth,
            sample_count: 1,
        }
    }
}

impl ResourceSettings {
    fn apply(&self, desc: &mut SgContextDesc) {
        desc.color_format = self.color_format as u32;
        desc.depth_format = self.depth_format as u32;
        desc.sample_count = self.sample_count as i32;
    }

    #[cfg(rokol_gfx = "glcore33")]
    fn create_context(&self) -> SgContextDesc {
        let mut desc = SgContextDesc::default();
        self.apply(&mut desc);
        // for OpenGL backend, we don't have to set context

        // TODO: support non-OpenGL backends
        // desc.gl.force_gles2 = sapp_gles2();
        // desc.metal.device = sapp_metal_get_device();
        // desc.metal.renderpass_descriptor_cb = sapp_metal_get_renderpass_descriptor;
        // desc.metal.drawable_cb = sapp_metal_get_drawable;
        // desc.d3d11.device = sapp_d3d11_get_device();
        // desc.d3d11.device_context = sapp_d3d11_get_device_context();
        // desc.d3d11.render_target_view_cb = sapp_d3d11_get_render_target_view;
        // desc.d3d11.depth_stencil_view_cb = sapp_d3d11_get_depth_stencil_view;
        // desc.wgpu.device = sapp_wgpu_get_device();
        // desc.wgpu.render_view_cb = sapp_wgpu_get_render_view;
        // desc.wgpu.resolve_view_cb = sapp_wgpu_get_resolve_view;
        // desc.wgpu.depth_stencil_view_cb = sapp_wgpu_get_depth_stencil_view;
        desc
    }

    fn init_gfx(&self) {
        let desc = rokol_ffi::gfx::sg_desc {
            context: self.create_context(),
            ..Default::default()
        };

        // FIXME: error at line 5038
        unsafe {
            rokol_ffi::gfx::sg_setup(&desc as *const _);
        }
    }
}

/// Set of SDL objects
///
/// Call `sg_sthudown` on end of your application.
pub struct WindowHandle {
    /// SDL lifetime (calls `SDL_QUIT` on drop)
    pub sdl: sdl2::Sdl,
    /// Lifetime of graphics (?)
    pub vid: sdl2::VideoSubsystem,
    /// SDL window lifetime (calls `SDL_DestroyWindow` on drop)
    pub win: sdl2::video::Window,
    /// SDL graphics lifetime (calls `SDL_GL_DeleteContext` on drop)
    #[cfg(rokol_gfx = "glcore33")]
    pub gcx: sdl2::video::GLContext,
}

impl WindowHandle {
    /// Call at the end of a frame to swap frame buffers
    #[cfg(rokol_gfx = "glcore33")]
    pub fn swap_window(&self) {
        self.win.gl_swap_window();
    }
}

#[derive(Debug)]
pub struct Init {
    pub name: String,
    pub w: u32,
    pub h: u32,
    pub settings: ResourceSettings,
}

impl Init {
    /// Initializes Rust-SDL2 and `rokol::gfx`
    ///
    /// I learned from this gist for using OpenGL with Sokol:
    /// <https://gist.github.com/sherjilozair/c0fa81250c1b8f5e4234b1588e755bca>
    #[cfg(rokol_gfx = "glcore33")]
    pub fn init(
        &self,
        mut f: impl FnMut(&mut sdl2::video::WindowBuilder),
    ) -> Result<WindowHandle, String> {
        // initialize SDL2 with selected graphics backend
        let sdl = sdl2::init()?;
        let vid = sdl.video()?;

        // GlCore33
        let attr = vid.gl_attr();
        attr.set_context_profile(sdl2::video::GLProfile::Core);
        attr.set_context_major_version(3);
        attr.set_context_minor_version(3);

        let win = {
            let mut b = vid.window(&self.name, self.w, self.h);
            b.opengl();
            f(&mut b);
            b.build().map_err(|e| e.to_string())?
        };

        let gcx = win.gl_create_context()?;

        // initialize rokol with selected graphics backend
        self.settings.init_gfx();

        Ok(WindowHandle { sdl, vid, win, gcx })
    }
}
