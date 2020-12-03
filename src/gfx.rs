//! Graphics

use {rokol_ffi::gfx as ffi, std::ffi::CString, std::mem::size_of};

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

/// Update strategy of buffers and images
///
/// # Kinds
///
/// * `Immutable`: Never be updated after creation
/// * `Dynamic`: Updated infrequently ("once after creation" to "quite often but not every frame")
/// * `Stream`: Updated each frame
///
/// The rendering backends use this hint to prevent that the
/// CPU needs to wait for the GPU when attempting to update
/// a resource that might be currently accessed by the GPU.
///
/// # Update frequency
///
/// Resource content is updated with the functions `sg_update_buffer()` or
/// `sg_append_buffer()` for buffer objects, and `sg_update_image()` for image
/// objects.
///
/// For the `sg_update_*()` functions, only one update is allowed per
/// frame and resource object, while `sg_append_buffer()` can be called
/// multiple times per frame on the same buffer. The application must update
/// all data required for rendering (this means that the update data can be
/// smaller than the resource size, if only a part of the overall resource
/// size is used for rendering, you only need to make sure that the data that
/// *is* used is valid).
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ResourceUsage {
    _Default = ffi::sg_usage__SG_USAGE_DEFAULT,
    Immutable = ffi::sg_usage_SG_USAGE_IMMUTABLE,
    Dynamic = ffi::sg_usage_SG_USAGE_DYNAMIC,
    Stream = ffi::sg_usage_SG_USAGE_STREAM,
    _ForceU32 = ffi::sg_usage__SG_USAGE_FORCE_U32,
    _Num = ffi::sg_usage__SG_USAGE_NUM,
}

/// Data type of a vertex component
///
/// Used to describe the layout of vertex data when creating a pipeline object.
///
/// # Portability of integer values
///
/// Only normalized integer formats (`*N`) is portable across all platforms.
///
/// The reason is that D3D11 cannot convert from non-normalized
/// formats to floating point inputs (only to integer inputs),
/// and WebGL2 / GLES2 don't support integer vertex shader inputs.
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

/// Fs | Vs
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ShaderStage {
    /// Fragment shader
    Fs = ffi::sg_shader_stage_SG_SHADERSTAGE_FS,
    /// Vertex shader
    Vs = ffi::sg_shader_stage_SG_SHADERSTAGE_VS,
    // _ForceU32 = ffi::sg_shader_stage__SG_SHADERSTAGE_FORCE_U32,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum UniformType {
    Float = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT,
    Float2 = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT2,
    Float3 = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT3,
    Float4 = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT4,
    Invalid = ffi::sg_uniform_type_SG_UNIFORMTYPE_INVALID,
    Mat4 = ffi::sg_uniform_type_SG_UNIFORMTYPE_MAT4,
    _ForceU32 = ffi::sg_uniform_type__SG_UNIFORMTYPE_FORCE_U32,
    _Num = ffi::sg_uniform_type__SG_UNIFORMTYPE_NUM,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum SamplerType {
    _Default = ffi::sg_sampler_type__SG_SAMPLERTYPE_DEFAULT,
    Float = ffi::sg_sampler_type_SG_SAMPLERTYPE_FLOAT,
    SInt = ffi::sg_sampler_type_SG_SAMPLERTYPE_SINT,
    UInt = ffi::sg_sampler_type_SG_SAMPLERTYPE_UINT,
}

/// Index | Vertex
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum BufferType {
    _Default = ffi::sg_buffer_type__SG_BUFFERTYPE_DEFAULT,
    Index = ffi::sg_buffer_type_SG_BUFFERTYPE_INDEXBUFFER,
    Vertex = ffi::sg_buffer_type_SG_BUFFERTYPE_VERTEXBUFFER,
    _ForceU32 = ffi::sg_buffer_type__SG_BUFFERTYPE_FORCE_U32,
    _Num = ffi::sg_buffer_type__SG_BUFFERTYPE_NUM,
}

/// UInt16 | UInt32
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum IndexType {
    _Default = ffi::sg_index_type__SG_INDEXTYPE_DEFAULT,
    None = ffi::sg_index_type_SG_INDEXTYPE_NONE,
    UInt16 = ffi::sg_index_type_SG_INDEXTYPE_UINT16,
    UInt32 = ffi::sg_index_type_SG_INDEXTYPE_UINT32,
    _ForceU32 = ffi::sg_index_type__SG_INDEXTYPE_FORCE_U32,
    _Num = ffi::sg_index_type__SG_INDEXTYPE_NUM,
}

/// 2D | 3D | Array | Cube
///
/// Basic type of an image object.
///
/// The image type is used in the `sg_image_desc.type` member when creating an image, and
/// in `sg_shader_image_desc` when describing a shader's texture sampler binding.
///
/// # Platform
///
/// 3D- and array-textures are not supported on the GLES2/WebGL backend
/// (use `sg_query_features().imagetype_3d` and `sg_query_features().imagetype_array` to check for
/// support).
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ImageType {
    _Default = ffi::sg_image_type__SG_IMAGETYPE_DEFAULT,
    /// 2D
    Dim2 = ffi::sg_image_type_SG_IMAGETYPE_2D,
    /// 3D
    Dim3 = ffi::sg_image_type_SG_IMAGETYPE_3D,
    Array = ffi::sg_image_type_SG_IMAGETYPE_ARRAY,
    Cube = ffi::sg_image_type_SG_IMAGETYPE_CUBE,
    _ForceU32 = ffi::sg_image_type__SG_IMAGETYPE_FORCE_U32,
    _Num = ffi::sg_image_type__SG_IMAGETYPE_NUM,
}

/// The texture coordinates wrapping mode when sampling a texture image
///
/// This is used in `sg_image_desc` when creating an image..
///
/// # Platform
///
/// `SG_WRAP_CLAMP_TO_BORDER` is not supported on all backends
/// and platforms. To check for support, call sg_query_features()
/// and check the "clamp_to_border" boolean in the returned
/// sg_features struct.
///
///
/// Platforms which don't support `SG_WRAP_CLAMP_TO_BORDER` will silently fall back
/// to `SG_WRAP_CLAMP_TO_EDGE` without a validation error.
///
/// Platforms which support clamp-to-border are:
///
///     - all desktop GL platforms
///     - Metal on macOS
///     - D3D11
///
/// Platforms which do not support clamp-to-border:
///
///     - GLES2/3 and WebGL/WebGL2
///     - Metal on iOS
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Wrap {
    _Default = ffi::sg_wrap__SG_WRAP_DEFAULT,
    /// [Platform]
    ClampToBorder = ffi::sg_wrap_SG_WRAP_CLAMP_TO_BORDER,
    ClampToEdge = ffi::sg_wrap_SG_WRAP_CLAMP_TO_EDGE,
    MirroredRepeat = ffi::sg_wrap_SG_WRAP_MIRRORED_REPEAT,
    Repeat = ffi::sg_wrap_SG_WRAP_REPEAT,
    _ForceU32 = ffi::sg_wrap__SG_WRAP_FORCE_U32,
    _Wrap = ffi::sg_wrap__SG_WRAP_NUM,
}

/// Pixel format
///
/// # Features
///
/// `sokol_gfx.h` basically uses the same pixel formats as WebGPU, since these
/// are supported on most newer GPUs. GLES2 and WebGL has a much smaller
/// subset of available pixel formats. Call `sg_query_pixelformat()` to check
/// at runtime if a pixel format supports the desired features.
///
/// # Naming convension
///
/// A pixelformat name consist of three parts:
///
///     - components (R, RG, RGB or RGBA)
///     - bit width per component (8, 16 or 32)
///     - component data type:
///         - unsigned normalized (no postfix)
///         - signed normalized (SN postfix)
///         - unsigned integer (UI postfix)
///         - signed integer (SI postfix)
///         - float (F postfix)
///
/// # Supported formats
///
/// Not all pixel formats can be used for everything, call `sg_query_pixelformat()`
/// to inspect the capabilities of a given pixelformat. The function returns
/// an `sg_pixelformat_info` struct with the following bool members:
///
///     - sample: the pixelformat can be sampled as texture at least with
///               nearest filtering
///     - filter: the pixelformat can be samples as texture with linear
///               filtering
///     - render: the pixelformat can be used for render targets
///     - blend:  blending is supported when using the pixelformat for
///               render targets
///     - msaa:   multisample-antialiasing is supported when using the
///               pixelformat for render targets
///     - depth:  the pixelformat can be used for depth-stencil attachments
///
/// When targeting GLES2/WebGL, the only safe formats to use
/// as texture are `SG_PIXELFORMAT_R8` and `SG_PIXELFORMAT_RGBA8`. For rendering
/// in GLES2/WebGL, only `SG_PIXELFORMAT_RGBA8` is safe. All other formats
/// must be checked via sg_query_pixelformats().
///
/// # Default pixel format
///
/// The default pixel format for texture images is `SG_PIXELFORMAT_RGBA8`.
///
/// The default pixel format for render target images is platform-dependent:
///     - for Metal and D3D11 it is `SG_PIXELFORMAT_BGRA8`
///     - for GL backends it is `SG_PIXELFORMAT_RGBA8`
///
/// This is mainly because of the default framebuffer which is setup outside
/// of `sokol_gfx.h`. On some backends, using BGRA for the default frame buffer
/// allows more efficient frame flips. For your own offscreen-render-targets,
/// use whatever renderable pixel format is convenient for you.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum PixelFormat {
    _Default,
    None,
    Rgba8,
    Rgb8,
    Rgba4,
    Rgb5,
    Rgb5a1,
    Rgb10a2,
    Rgba32f,
    Rgba16f,
    R32F,
    R16F,
    L8,
    Dxt1,
    Dxt3,
    Dxt5,
    Depth,
    DepthStencil,
    Pvrtc2Rgb,
    Pvrtc4Rgb,
    Pvrtc2Rgba,
    Pvrtc4Rgba,
    Etc2Rgb8,
    Etc2SRgb8,
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
// Baked resource types compiled into immutable ones

/// [Resource] Handle (ID) of vertex | index buffer
pub type Buffer = ffi::sg_buffer;
pub type BufferInfo = ffi::sg_buffer_info;
pub type BufferLayoutDesc = ffi::sg_buffer_layout_desc;
pub type BufferDesc = ffi::sg_buffer_desc;

/// [Resource] Handle (ID) of pipeline object
pub type Pipeline = ffi::sg_pipeline;
pub type PipelineInfo = ffi::sg_pipeline_info;
pub type PipelineDesc = ffi::sg_pipeline_desc;

/// [Resource] Hadnle (ID) of image
pub type Image = ffi::sg_image;
pub type ImageContent = ffi::sg_image_content;
pub type ImageDesc = ffi::sg_image_desc;
pub type ImageInfo = ffi::sg_image_info;

/// [Resource] Handle (ID) of shader
pub type Shader = ffi::sg_shader;
pub type ShaderAttrDesc = ffi::sg_shader_attr_desc;
pub type ShaderDesc = ffi::sg_shader_desc;
pub type ShaderImageDesc = ffi::sg_shader_image_desc;
pub type ShaderInfo = ffi::sg_shader_info;
pub type ShaderStageDesc = ffi::sg_shader_stage_desc;

/// [Resource] Handle(ID) of pass
pub type Pass = ffi::sg_pass;
pub type PassDesc = ffi::sg_pass_desc;
pub type PassInfo = ffi::sg_pass_info;

// --------------------------------------------------------------------------------

pub type ColorAttachmentAction = ffi::sg_color_attachment_action;
pub type Context = ffi::sg_context;
pub type ContextDesc = ffi::sg_context_desc;

pub type DepthAttachmentAction = ffi::sg_depth_attachment_action;
pub type DepthStencilState = ffi::sg_depth_stencil_state;
pub type RasterizerState = ffi::sg_rasterizer_state;

pub type Desc = ffi::sg_desc;
pub type Features = ffi::sg_features;

pub type LayoutDesc = ffi::sg_layout_desc;
pub type Limits = ffi::sg_limits;

pub type PixelFormatInfo = ffi::sg_pixelformat_info;

pub type ShaderUniformBlockDesc = ffi::sg_shader_uniform_block_desc;
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

pub fn apply_pipeline(pip: Pipeline) {
    unsafe {
        ffi::sg_apply_pipeline(pip);
    }
}

pub fn apply_bindings(bind: &Bindings) {
    unsafe {
        ffi::sg_apply_bindings(bind);
    }
}

pub fn apply_uniforms<T>(stage: ShaderStage, ub_index: u32, data: &[T]) {
    unsafe {
        ffi::sg_apply_uniforms(
            stage as u32,
            ub_index as i32,
            data.as_ptr() as *mut _,
            (size_of::<T>() * data.len()) as i32,
        );
    }
}

pub fn draw(base_elem: u32, n_elems: u32, n_instances: u32) {
    unsafe {
        ffi::sg_draw(base_elem as i32, n_elems as i32, n_instances as i32);
    }
}

// ----------------------------------------
// Resource creation

// Do not use them until you call `rokol_gfx::setup` in `init`

// make = alloc + init

pub fn make_buffer(desc: &BufferDesc) -> Buffer {
    unsafe { ffi::sg_make_buffer(desc) }
}

pub fn make_image(desc: &ImageDesc) -> Image {
    unsafe { ffi::sg_make_image(desc) }
}

pub fn make_pass(desc: &PassDesc) -> Pass {
    unsafe { ffi::sg_make_pass(desc) }
}

pub fn make_pipeline(desc: &PipelineDesc) -> Pipeline {
    unsafe { ffi::sg_make_pipeline(desc) }
}

pub fn make_shader(desc: &ShaderDesc) -> Shader {
    unsafe { ffi::sg_make_shader(desc) }
}

// alloc

pub fn alloc_buffer() -> Buffer {
    unsafe { ffi::sg_alloc_buffer() }
}

pub fn alloc_image() -> Image {
    unsafe { ffi::sg_alloc_image() }
}

pub fn alloc_pass() -> Pass {
    unsafe { ffi::sg_alloc_pass() }
}

pub fn alloc_pipeline() -> Pipeline {
    unsafe { ffi::sg_alloc_pipeline() }
}

pub fn alloc_shader() -> Shader {
    unsafe { ffi::sg_alloc_shader() }
}

// init

pub fn init_buffer(id: Buffer, desc: &BufferDesc) {
    unsafe { ffi::sg_init_buffer(id, desc) }
}

pub fn init_image(id: Image, desc: &ImageDesc) {
    unsafe { ffi::sg_init_image(id, desc) }
}

pub fn init_pass(id: Pass, desc: &PassDesc) {
    unsafe { ffi::sg_init_pass(id, desc) }
}

pub fn init_pipeline(id: Pipeline, desc: &PipelineDesc) {
    unsafe { ffi::sg_init_pipeline(id, desc) }
}

pub fn init_shader(id: Shader, desc: &ShaderDesc) {
    unsafe { ffi::sg_init_shader(id, desc) }
}

// ----------------------------------------
// Resource deletion

// rm (destrory) = dealloc + uninit

pub fn rm_buffer(id: Buffer) {
    unsafe {
        ffi::sg_destroy_buffer(id);
    }
}

pub fn rm_image(id: Image) {
    unsafe {
        ffi::sg_destroy_image(id);
    }
}

pub fn rm_pass(id: Pass) {
    unsafe {
        ffi::sg_destroy_pass(id);
    }
}

pub fn rm_pipeline(id: Pipeline) {
    unsafe {
        ffi::sg_destroy_pipeline(id);
    }
}

pub fn rm_shader(id: Shader) {
    unsafe {
        ffi::sg_destroy_shader(id);
    }
}

// dealloc

pub fn dealloc_buffer(id: Buffer) {
    unsafe {
        ffi::sg_dealloc_buffer(id);
    }
}

pub fn dealloc_image(id: Image) {
    unsafe {
        ffi::sg_dealloc_image(id);
    }
}

pub fn dealloc_pass(id: Pass) {
    unsafe {
        ffi::sg_dealloc_pass(id);
    }
}

pub fn dealloc_pipeline(id: Pipeline) {
    unsafe {
        ffi::sg_dealloc_pipeline(id);
    }
}

pub fn dealloc_shader(id: Shader) {
    unsafe {
        ffi::sg_dealloc_shader(id);
    }
}

// --------------------------------------------------------------------------------
// Helpers

/// [Non-Sokol] Helper for making shaders
///
/// Caller must ensure the shader strings are null-terminated!
pub unsafe fn shader_desc(vs: &str, fs: &str) -> ShaderDesc {
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
    desc
}

/// [Non-Sokol] Helper for creating index buffer
pub fn ibuf_desc<T>(buf: &[T], usage: ResourceUsage, label: &str) -> BufferDesc {
    buf_desc(buf, BufferType::Index, usage, label)
}

/// [Non-Sokol] Helper for creating vertex buffer
pub fn vbuf_desc<T>(buf: &[T], usage: ResourceUsage, label: &str) -> BufferDesc {
    buf_desc(buf, BufferType::Vertex, usage, label)
}

fn buf_desc<T>(
    buf: &[T],
    buffer_type: BufferType,
    usage: ResourceUsage,
    label: &str,
) -> BufferDesc {
    let size = (std::mem::size_of::<T>() * buf.len()) as i32;
    ffi::sg_buffer_desc {
        size,
        content: buf.as_ptr() as *mut _,
        type_: buffer_type as u32,
        usage: usage as u32,
        label: if label == "" {
            std::ptr::null_mut()
        } else {
            let label = CString::new(label).expect("Unable to create CString in BufferDesc::new");
            label.as_ptr() as *mut _
        },
        ..Default::default()
    }
}
