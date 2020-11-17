//! `rokol::gfx`, graphics

use {rokol_ffi::gfx as ffi, std::ffi::CString};

pub fn setup(desc: &mut Desc) {
    unsafe {
        ffi::sg_setup(desc as *const _ as *mut _);
    }
}

macro_rules! raw_access {
    ($name:ident, $t:path) => {
        impl $name {
            #[allow(dead_code)]
            #[inline]
            fn raw(&self) -> &$t {
                &self.raw
            }

            #[allow(dead_code)]
            #[inline]
            fn raw_mut(&mut self) -> &mut $t {
                &mut self.raw
            }
        }
    };
}

// --------------------------------------------------------------------------------
// Enums

/// Actions to be performed at the start of a rendering pass in `begin_pass` or `begindefault_pass`
///
/// `sg_action` in `sokol_gfx.h`.
///
/// A separate action and clear values can be defined for each
/// color attachment, and for the depth-stencil attachment.
///
/// # The default clear values
///
/// - SG_DEFAULT_CLEAR_RED:     0.5f
/// - SG_DEFAULT_CLEAR_GREEN:   0.5f
/// - SG_DEFAULT_CLEAR_BLUE:    0.5f
/// - SG_DEFAULT_CLEAR_ALPHA:   1.0f
/// - SG_DEFAULT_CLEAR_DEPTH:   1.0f
/// - SG_DEFAULT_CLEAR_STENCIL: 0
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum PassActionKind {
    _Default = ffi::sg_action__SG_ACTION_DEFAULT,
    Clear = ffi::sg_action_SG_ACTION_CLEAR,
    Load = ffi::sg_action_SG_ACTION_LOAD,
    DontCare = ffi::sg_action_SG_ACTION_DONTCARE,
}

/// `sg_usage` in `sokol_gfx.h`
///
/// A resource usage hint describing the update strategy of
/// buffers and images. This is used in the sg_buffer_desc.usage
/// and sg_image_desc.usage members when creating buffers
/// and images:
///
/// SG_USAGE_IMMUTABLE:     the resource will never be updated with
///                         new data, instead the content of the
///                         resource must be provided on creation
/// SG_USAGE_DYNAMIC:       the resource will be updated infrequently
///                         with new data (this could range from "once
///                         after creation", to "quite often but not
///                         every frame")
/// SG_USAGE_STREAM:        the resource will be updated each frame
///                         with new content
///
/// The rendering backends use this hint to prevent that the
/// CPU needs to wait for the GPU when attempting to update
/// a resource that might be currently accessed by the GPU.
///
/// Resource content is updated with the functions sg_update_buffer() or
/// sg_append_buffer() for buffer objects, and sg_update_image() for image
/// objects. For the sg_update_*() functions, only one update is allowed per
/// frame and resource object, while sg_append_buffer() can be called
/// multiple times per frame on the same buffer. The application must update
/// all data required for rendering (this means that the update data can be
/// smaller than the resource size, if only a part of the overall resource
/// size is used for rendering, you only need to make sure that the data that
/// *is* used is valid).
///
/// The default usage is SG_USAGE_IMMUTABLE.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ResourceUsage {
    _Default = ffi::sg_usage__SG_USAGE_DEFAULT,
    Immutable = ffi::sg_usage_SG_USAGE_IMMUTABLE,
    Dynamic = ffi::sg_usage_SG_USAGE_DYNAMIC,
    Stream = ffi::sg_usage_SG_USAGE_STREAM,
}

// #[repr(C)]
// #[derive(Copy, Clone, Debug)]
// pub enum SgPixelFormat {
//     // _Default,
//     None,
//     RGBA8,
//     RGB8,
//     RGBA4,
//     RGB5,
//     RGB5A1,
//     RGB10A2,
//     RGBA32F,
//     RGBA16F,
//     R32F,
//     R16F,
//     L8,
//     DXT1,
//     DXT3,
//     DXT5,
//     Depth,
//     DepthStencil,
//     PVRTC2RGB,
//     PVRTC4RGB,
//     PVRTC2RGBA,
//     PVRTC4RGBA,
//     ETC2RGB8,
//     ETC2SRGB8,
// }

/*
    sg_vertex_format

    The data type of a vertex component. This is used to describe
    the layout of vertex data when creating a pipeline object.
*/

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum VertexFormat {
    Inalid = ffi::sg_vertex_format_SG_VERTEXFORMAT_INVALID,
    Float = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT,
    Float2 = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT2,
    Float3 = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT3,
    Float4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT4,
    Byte4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_BYTE4,
    Byte4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_BYTE4N,
    UByte4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_UBYTE4,
    UByte4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_UBYTE4N,
    Short2 = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT2,
    Short2N = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT2N,
    UShort2N = ffi::sg_vertex_format_SG_VERTEXFORMAT_USHORT2N,
    Short4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT4,
    Short4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT4N,
    UShort4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_USHORT4N,
    Uint10N2 = ffi::sg_vertex_format_SG_VERTEXFORMAT_UINT10_N2,
    _Num = ffi::sg_vertex_format__SG_VERTEXFORMAT_NUM,
    _ForceU32 = ffi::sg_vertex_format__SG_VERTEXFORMAT_FORCE_U32,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ShaderStage {
    /// Fragment shader
    Fs = ffi::sg_shader_stage_SG_SHADERSTAGE_FS,
    /// Vertex shader
    Vs = ffi::sg_shader_stage_SG_SHADERSTAGE_VS,
    // _ForceU32 = ffi::sg_shader_stage__SG_SHADERSTAGE_FORCE_U32,
}

// --------------------------------------------------------------------------------
// Wrapped structs

#[derive(Debug, Default)]
pub struct PassAction {
    raw: ffi::sg_pass_action,
}

impl AsRef<ffi::sg_pass_action> for PassAction {
    fn as_ref(&self) -> &ffi::sg_pass_action {
        &self.raw
    }
}

impl PassAction {
    pub fn raw(&self) -> &ffi::sg_pass_action {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut ffi::sg_pass_action {
        &mut self.raw
    }

    pub fn clear(color: impl Into<[f32; 4]>) -> Self {
        let mut raw = ffi::sg_pass_action::default();
        raw.colors[0] = ColorAttachmentAction {
            action: PassActionKind::Clear as u32,
            val: color.into(),
        };
        Self { raw }
    }
}

// --------------------------------------------------------------------------------
// Re-exports from the FFI

pub type AttachmentDesc = ffi::sg_attachment_desc;
pub type Bindings = ffi::sg_bindings;
pub type BlendState = ffi::sg_blend_state;

// --------------------------------------------------------------------------------
// Resource objects

pub type Buffer = ffi::sg_buffer;
pub type BufferInfo = ffi::sg_buffer_info;
pub type BufferLayoutDesc = ffi::sg_buffer_layout_desc;

#[derive(Debug)]
pub struct BufferDesc {
    raw: ffi::sg_buffer_desc,
}

raw_access!(BufferDesc, ffi::sg_buffer_desc);

impl BufferDesc {
    pub fn new<T>(buf: &[T], usage: ResourceUsage, label: &str) -> Self {
        let size = (std::mem::size_of::<T>() * buf.len()) as i32;
        Self {
            raw: ffi::sg_buffer_desc {
                size,
                content: buf.as_ptr() as *mut _,
                usage: usage as u32,
                label: if label == "" {
                    std::ptr::null_mut()
                } else {
                    let label =
                        CString::new(label).expect("Unable to create CString in BufferDesc::new");
                    label.as_ptr() as *mut _
                },
                ..Default::default()
            },
        }
    }
}

pub type Pipeline = ffi::sg_pipeline;
pub type PipelineInfo = ffi::sg_pipeline_info;
pub type PipelineDesc = ffi::sg_pipeline_desc;

// pub struct PipelineDesc {
//     raw: ffi::sg_pipeline_desc,
// }
//
// raw_access!(PipelineDesc, ffi::sg_pipeline_desc);
//
// impl PipelineDesc {
//     pub fn new
// }

// --------------------------------------------------------------------------------

pub type ColorAttachmentAction = ffi::sg_color_attachment_action;
pub type Context = ffi::sg_context;
pub type ContextDesc = ffi::sg_context_desc;

pub type DepthAttachmentAction = ffi::sg_depth_attachment_action;
pub type DepthStencilState = ffi::sg_depth_stencil_state;
pub type RasterizerState = ffi::sg_rasterizer_state;

pub type Desc = ffi::sg_desc;
pub type Features = ffi::sg_features;

pub type Image = ffi::sg_image;
pub type ImageContent = ffi::sg_image_content;
pub type ImageDesc = ffi::sg_image_desc;
pub type ImageInfo = ffi::sg_image_info;

pub type LayoutDesc = ffi::sg_layout_desc;
pub type Limits = ffi::sg_limits;

pub type Pass = ffi::sg_pass;
pub type PassDesc = ffi::sg_pass_desc;
pub type PassInfo = ffi::sg_pass_info;

pub type PixelFormatInfo = ffi::sg_pixelformat_info;

pub type Shader = ffi::sg_shader;
pub type ShaderAttrDesc = ffi::sg_shader_attr_desc;
pub type ShaderDesc = ffi::sg_shader_desc;
pub type ShaderImageDesc = ffi::sg_shader_image_desc;
pub type ShaderInfo = ffi::sg_shader_info;
pub type ShaderStageDesc = ffi::sg_shader_stage_desc;

pub type ShaderUniformBlock = ffi::sg_shader_uniform_block_desc;
pub type ShaderUniformDesc = ffi::sg_shader_uniform_desc;

pub type SlotInfo = ffi::sg_slot_info;
pub type AttachmentAction = ffi::sg_stencil_attachment_action;
pub type StencilState = ffi::sg_stencil_state;
pub type SubimageContent = ffi::sg_subimage_content;
pub type TraceHooks = ffi::sg_trace_hooks;
pub type VertexAttrDesc = ffi::sg_vertex_attr_desc;

pub type GlContextDesc = ffi::sg_gl_context_desc;
pub type D3D11ContextDesc = ffi::sg_d3d11_context_desc;
pub type MtlContextDesc = ffi::sg_mtl_context_desc;
pub type WgpuContextDesc = ffi::sg_wgpu_context_desc;

// --------------------------------------------------------------------------------
// Functions

pub fn begin_default_pass(pa: &impl AsRef<ffi::sg_pass_action>, w: u32, h: u32) {
    unsafe {
        ffi::sg_begin_default_pass(pa.as_ref(), w as i32, h as i32);
    }
}

pub fn end_pass() {
    unsafe {
        ffi::sg_end_pass();
    }
}

pub fn commit() {
    unsafe {
        ffi::sg_commit();
    }
}

pub fn apply_pipeline(&pip: &Pipeline) {
    unsafe {
        ffi::sg_apply_pipeline(pip);
    }
}

pub fn apply_bindings(bind: &Bindings) {
    unsafe {
        ffi::sg_apply_bindings(bind);
    }
}

pub fn draw(base_elem: u32, n_elems: u32, n_instances: u32) {
    unsafe {
        ffi::sg_draw(base_elem as i32, n_elems as i32, n_instances as i32);
    }
}

// ----------------------------------------
// Resource creation
//
// Be careful to not use them until you call `rokol_gfx::setup` in `init`

pub fn make_buffer(desc: &BufferDesc) -> Buffer {
    unsafe { ffi::sg_make_buffer(desc.raw()) }
}

pub fn make_pipeline(desc: &PipelineDesc) -> Pipeline {
    unsafe { ffi::sg_make_pipeline(desc) }
}

pub fn make_shader(desc: &ShaderDesc) -> Shader {
    unsafe { ffi::sg_make_shader(desc) }
}

/// [Non-Sokol] Helper for making shaders
///
/// Caller must ensure the shader strings are null-terminated!
pub unsafe fn make_shader_static(vs: &str, fs: &str) -> Shader {
    let mut desc = ShaderDesc::default();
    desc.vs = ShaderStageDesc {
        source: vs.as_ptr() as *mut _,
        uniform_blocks: [Default::default(); ffi::SG_MAX_SHADERSTAGE_UBS as usize],
        images: [Default::default(); ffi::SG_MAX_SHADERSTAGE_IMAGES as usize],
        ..Default::default()
    };
    desc.fs = ShaderStageDesc {
        source: fs.as_ptr() as *mut _,
        uniform_blocks: [Default::default(); ffi::SG_MAX_SHADERSTAGE_UBS as usize],
        images: [Default::default(); ffi::SG_MAX_SHADERSTAGE_IMAGES as usize],
        ..Default::default()
    };

    self::make_shader(&desc)
}
