/*!

Graphics ([`FFI`])

[`FFI`]: rokol_ffi::gfx

# Resource types

[`BakedResource`] implementations:

* [`Buffer`]: index or vertex buffer
* [`Image`]: 2D or 3D image
* [`Pass`]: screen or offscreen rendering pass
* [`Pipeline`]: vertex-layouts, shader and render states
* [`Shader`]: vertex and fragment shaders with shader-parameter declarations

Be sure to specify uniform names when making [`Shader`].

# Render loop

For example, for one frame with one screen rendering pass:

* [`begin_default_pass`] (screen rendering pass)
    * [`viewport`] and [`scissor`]
    * [`apply_pipeline`] (vertex-layouts, shader and render states)
    * [`apply_bindings`] ([`Bindings`]: vertex and index buffer and images)
    * [`apply_uniforms`] (set shader uniform with an index)
    * [`draw`]
* [`end_pass`]
* [`commit`]

# References

* Sokol articles (The Brain Dump)
    * [A Tour of sokol_gfx.h](https://floooh.github.io/2017/07/29/sokol-gfx-tour.html) (2017)
    * [A small sokol_gfx.h API update](https://floooh.github.io/2019/01/12/sokol-apply-pipeline.html) (2019)
    * [Sokol headers: spring 2020 update](https://floooh.github.io/2020/04/26/sokol-spring-2020-update.html) (2020)
* [Learn OpenGL](https://learnopengl.com/)
* [Learn OpenGL Examples (with Sokol in C)](https://www.geertarien.com/learnopengl-examples-html5/)
* [zig-renderkit](https://github.com/prime31/zig-renderkit)

*/

use {rokol_ffi::gfx as ffi, std::ffi::CString, std::mem::size_of};

/// Should be called from [`crate::app::RApp::init`]
pub fn setup(desc: &mut SetupDesc) {
    unsafe {
        ffi::sg_setup(desc as *const _ as *mut _);
    }
}

/// [`setup`] parameter, which is created from [`crate::glue::app_desc`]
pub type SetupDesc = ffi::sg_desc;

// --------------------------------------------------------------------------------
// Resource enums

/// Actions to be performed at the start of a rendering pass in [`begin_pass`] or [`begin_default_pass`]
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

/// Mat4 | Float | Float2 | Float3 | Float4
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

/// Float | SInt | UInt
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum SamplerType {
    _Default = ffi::sg_sampler_type__SG_SAMPLERTYPE_DEFAULT,
    Float = ffi::sg_sampler_type_SG_SAMPLERTYPE_FLOAT,
    SInt = ffi::sg_sampler_type_SG_SAMPLERTYPE_SINT,
    UInt = ffi::sg_sampler_type_SG_SAMPLERTYPE_UINT,
}

// --------------------------------------------------------------------------------
// Binding enums

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

/// Common subset of 3D primitive types supported across all 3D APIs. Field of [`PipelineDesc`].
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum PrimitiveType {
    _Default = ffi::sg_primitive_type__SG_PRIMITIVETYPE_DEFAULT,
    _ForuceU32 = ffi::sg_primitive_type__SG_PRIMITIVETYPE_FORCE_U32,
    _Num = ffi::sg_primitive_type__SG_PRIMITIVETYPE_NUM,
    Lines = ffi::sg_primitive_type_SG_PRIMITIVETYPE_LINES,
    LinesStrip = ffi::sg_primitive_type_SG_PRIMITIVETYPE_LINE_STRIP,
    Points = ffi::sg_primitive_type_SG_PRIMITIVETYPE_POINTS,
    Triangles = ffi::sg_primitive_type_SG_PRIMITIVETYPE_TRIANGLES,
    TrianglesStrip = ffi::sg_primitive_type_SG_PRIMITIVETYPE_TRIANGLE_STRIP,
}

// --------------------------------------------------------------------------------
// Image enums

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

/// The filtering mode when sampling a texture image
///
/// This is used in the `sg_image_desc.min_filter` and `sg_image_desc.mag_filter`
/// members when creating an image object.
///
/// The default filter mode is SG_FILTER_NEAREST.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Filter {
    Linear = ffi::sg_filter_SG_FILTER_LINEAR,
    LinearMipmap = ffi::sg_filter_SG_FILTER_LINEAR_MIPMAP_LINEAR,
    LinearMipmapNearest = ffi::sg_filter_SG_FILTER_LINEAR_MIPMAP_NEAREST,
    Nearest = ffi::sg_filter_SG_FILTER_NEAREST,
    NearestMipmapLinear = ffi::sg_filter_SG_FILTER_NEAREST_MIPMAP_LINEAR,
    NearestMipmapNearest = ffi::sg_filter_SG_FILTER_NEAREST_MIPMAP_NEAREST,
    _Default = ffi::sg_filter__SG_FILTER_DEFAULT,
    _ForceU32 = ffi::sg_filter__SG_FILTER_FORCE_U32,
    _Num = ffi::sg_filter__SG_FILTER_NUM,
}

/// The texture coordinates wrapping mode when sampling a texture image
///
/// This is used in `sg_image_desc` when creating an image..
///
/// # Platform
///
/// `SG_WRAP_CLAMP_TO_BORDER` is not supported on all backends
/// and platforms. To check for support, call sg_query_features()
/// and check the "clamp_to_border" nitboolean in the returned
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
    /// (Platform)
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
#[repr(u32)]
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
// Rendering enums

/// `"`, `!=`, `>`, `>=`, `<`, `<=`, `true`, `false`
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum CompareFunc {
    Always = ffi::sg_compare_func_SG_COMPAREFUNC_ALWAYS,
    Eq = ffi::sg_compare_func_SG_COMPAREFUNC_EQUAL,
    Greater = ffi::sg_compare_func_SG_COMPAREFUNC_GREATER,
    GreaterEq = ffi::sg_compare_func_SG_COMPAREFUNC_GREATER_EQUAL,
    Less = ffi::sg_compare_func_SG_COMPAREFUNC_LESS,
    LessEq = ffi::sg_compare_func_SG_COMPAREFUNC_LESS_EQUAL,
    Never = ffi::sg_compare_func_SG_COMPAREFUNC_NEVER,
    NotEq = ffi::sg_compare_func_SG_COMPAREFUNC_NOT_EQUAL,
    _Default = ffi::sg_compare_func__SG_COMPAREFUNC_DEFAULT,
    _ForceU32 = ffi::sg_compare_func__SG_COMPAREFUNC_FORCE_U32,
    _Num = ffi::sg_compare_func__SG_COMPAREFUNC_NUM,
}

/// Front | Back | None
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum CullMode {
    Back = ffi::sg_cull_mode_SG_CULLMODE_BACK,
    Front = ffi::sg_cull_mode_SG_CULLMODE_FRONT,
    None = ffi::sg_cull_mode_SG_CULLMODE_NONE,
    _Default = ffi::sg_cull_mode__SG_CULLMODE_DEFAULT,
    _ForuceU32 = ffi::sg_cull_mode__SG_CULLMODE_FORCE_U32,
    _Num = ffi::sg_cull_mode__SG_CULLMODE_NUM,
}

/// CCW | CW |
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FaceWinding {
    _Default = ffi::sg_face_winding__SG_FACEWINDING_DEFAULT,
    Ccw = ffi::sg_face_winding_SG_FACEWINDING_CCW,
    Cw = ffi::sg_face_winding_SG_FACEWINDING_CW,
    _Num = ffi::sg_face_winding__SG_FACEWINDING_NUM,
    _ForceU32 = ffi::sg_face_winding__SG_FACEWINDING_FORCE_U32,
}

/// Pass action
///
/// Wraps [`ffi::sg_pass_action`] just to add methods without trait.
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

/// Vertex/index buffer and image slots
pub type Bindings = ffi::sg_bindings;

pub type BlendState = ffi::sg_blend_state;

// --------------------------------------------------------------------------------
// Baked resource types compiled into immutable ones

/// [`Buffer`] | [`Image`] | [`Pipeline`] | [`Pass`] | [`Shader`]
///
/// Resource object baked into immutable state.
pub trait BakedResource {
    type Id;
    type Desc;
    /// `alloc` + `init`
    fn create(desc: &Self::Desc) -> Self::Id;
    fn alloc() -> Self::Id;
    /// Initializes an allocated resource object
    fn init(id: Self::Id, desc: &Self::Desc);

    /// `uninit` + `dealloc`
    fn destroy(id: Self::Id);
    fn uninit(id: Self::Id);
    /// Deallocates an uninitlized resouce object
    fn dealloc(id: Self::Id);
}

/// (Resource) Handle (ID) of vertex | index buffer
///
/// Created from [`BufferDesc`] via [`BakedResource::create`].
pub type Buffer = ffi::sg_buffer;
pub type BufferDesc = ffi::sg_buffer_desc;
pub type BufferInfo = ffi::sg_buffer_info;
pub type BufferLayoutDesc = ffi::sg_buffer_layout_desc;

impl BakedResource for Buffer {
    type Id = Buffer;
    type Desc = BufferDesc;

    fn create(desc: &Self::Desc) -> Self::Id {
        unsafe { ffi::sg_make_buffer(desc) }
    }

    fn alloc() -> Self::Id {
        unsafe { ffi::sg_alloc_buffer() }
    }

    fn init(id: Self::Id, desc: &Self::Desc) {
        unsafe { ffi::sg_init_buffer(id, desc) }
    }

    fn destroy(id: Self::Id) {
        unsafe {
            ffi::sg_destroy_buffer(id);
        }
    }

    fn uninit(id: Self::Id) {
        unsafe {
            ffi::sg_uninit_buffer(id);
        }
    }

    fn dealloc(id: Self::Id) {
        unsafe {
            ffi::sg_dealloc_buffer(id);
        }
    }
}

/// (Resource) Handle (ID) of image
pub type Image = ffi::sg_image;
pub type ImageDesc = ffi::sg_image_desc;
pub type ImageContent = ffi::sg_image_content;
pub type ImageInfo = ffi::sg_image_info;

impl BakedResource for Image {
    type Id = Image;
    type Desc = ImageDesc;

    fn create(desc: &Self::Desc) -> Self::Id {
        unsafe { ffi::sg_make_image(desc) }
    }

    fn alloc() -> Self::Id {
        unsafe { ffi::sg_alloc_image() }
    }

    fn init(id: Self::Id, desc: &Self::Desc) {
        unsafe { ffi::sg_init_image(id, desc) }
    }

    fn destroy(id: Self::Id) {
        unsafe {
            ffi::sg_destroy_image(id);
        }
    }

    fn uninit(id: Self::Id) {
        unsafe {
            ffi::sg_uninit_image(id);
        }
    }

    fn dealloc(id: Self::Id) {
        unsafe {
            ffi::sg_dealloc_image(id);
        }
    }
}

/// (Resource) Handle (ID) of pipeline object: vertex layouts, shader and render states
///
/// Created from [`PipelineDesc`] via [`BakedResource::create`].
pub type Pipeline = ffi::sg_pipeline;

pub type PipelineInfo = ffi::sg_pipeline_info;
pub type PipelineDesc = ffi::sg_pipeline_desc;

impl BakedResource for Pipeline {
    type Id = Pipeline;
    type Desc = PipelineDesc;

    fn create(desc: &Self::Desc) -> Self::Id {
        unsafe { ffi::sg_make_pipeline(desc) }
    }

    fn alloc() -> Self::Id {
        unsafe { ffi::sg_alloc_pipeline() }
    }

    fn init(id: Self::Id, desc: &Self::Desc) {
        unsafe { ffi::sg_init_pipeline(id, desc) }
    }

    fn destroy(id: Self::Id) {
        unsafe {
            ffi::sg_destroy_pipeline(id);
        }
    }

    fn uninit(id: Self::Id) {
        unsafe {
            ffi::sg_uninit_pipeline(id);
        }
    }

    fn dealloc(id: Self::Id) {
        unsafe {
            ffi::sg_dealloc_pipeline(id);
        }
    }
}

/// (Resource) Handle(ID) of rendering pass
///
/// Created from [`PassDesc`] via [`BakedResource::create`].
pub type Pass = ffi::sg_pass;
pub type PassDesc = ffi::sg_pass_desc;
pub type PassInfo = ffi::sg_pass_info;

impl BakedResource for Pass {
    type Id = Pass;
    type Desc = PassDesc;

    fn create(desc: &Self::Desc) -> Self::Id {
        unsafe { ffi::sg_make_pass(desc) }
    }

    fn alloc() -> Self::Id {
        unsafe { ffi::sg_alloc_pass() }
    }

    fn init(id: Self::Id, desc: &Self::Desc) {
        unsafe { ffi::sg_init_pass(id, desc) }
    }

    fn destroy(id: Self::Id) {
        unsafe {
            ffi::sg_destroy_pass(id);
        }
    }

    fn uninit(id: Self::Id) {
        unsafe {
            ffi::sg_uninit_pass(id);
        }
    }

    fn dealloc(id: Self::Id) {
        unsafe {
            ffi::sg_dealloc_pass(id);
        }
    }
}

/// (Resource) Handle (ID) of shader
///
/// Created from [`ShaderDesc`] via [`BakedResource::create`].
pub type Shader = ffi::sg_shader;
pub type ShaderAttrDesc = ffi::sg_shader_attr_desc;
pub type ShaderDesc = ffi::sg_shader_desc;
pub type ShaderImageDesc = ffi::sg_shader_image_desc;
pub type ShaderInfo = ffi::sg_shader_info;
pub type ShaderStageDesc = ffi::sg_shader_stage_desc;

impl BakedResource for Shader {
    type Id = Shader;
    type Desc = ShaderDesc;

    fn create(desc: &Self::Desc) -> Self::Id {
        unsafe { ffi::sg_make_shader(desc) }
    }

    fn alloc() -> Self::Id {
        unsafe { ffi::sg_alloc_shader() }
    }

    fn init(id: Self::Id, desc: &Self::Desc) {
        unsafe { ffi::sg_init_shader(id, desc) }
    }

    fn destroy(id: Self::Id) {
        unsafe {
            ffi::sg_destroy_shader(id);
        }
    }

    fn uninit(id: Self::Id) {
        unsafe {
            ffi::sg_uninit_shader(id);
        }
    }

    fn dealloc(id: Self::Id) {
        unsafe {
            ffi::sg_dealloc_shader(id);
        }
    }
}

// --------------------------------------------------------------------------------

pub type ColorAttachmentAction = ffi::sg_color_attachment_action;
pub type Context = ffi::sg_context;
pub type ContextDesc = ffi::sg_context_desc;

pub type DepthAttachmentAction = ffi::sg_depth_attachment_action;
pub type DepthStencilState = ffi::sg_depth_stencil_state;
pub type RasterizerState = ffi::sg_rasterizer_state;

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

/// Screen rendering pass
pub fn begin_default_pass(pa: &impl AsRef<ffi::sg_pass_action>, w: u32, h: u32) {
    unsafe {
        ffi::sg_begin_default_pass(pa.as_ref(), w as i32, h as i32);
    }
}

/// Offscreeen rendering pass
pub fn begin_pass(pass: Pass, pa: &impl AsRef<ffi::sg_pass_action>) {
    unsafe {
        ffi::sg_begin_pass(pass, pa.as_ref());
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

/// Applies [`Pipeline`]: vertex-layouts, shader and render states)
pub fn apply_pipeline(pip: Pipeline) {
    unsafe {
        ffi::sg_apply_pipeline(pip);
    }
}

/// Applies buffer [`Bindings`]: vertex/index buffer and images
pub fn apply_bindings(bind: &Bindings) {
    unsafe {
        ffi::sg_apply_bindings(bind);
    }
}

/// Applies uniform data to shader
///
/// * `ub-index`: uniform block index
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

/// `draw(base_elems, n_elems, n_instances)`
pub fn draw(base_elem: u32, n_elems: u32, n_instances: u32) {
    unsafe {
        ffi::sg_draw(base_elem as i32, n_elems as i32, n_instances as i32);
    }
}

/// Must be called inside a rendering pass
pub fn scissor(x: u32, y: u32, w: u32, h: u32) {
    unsafe {
        // origin_top_left: true
        ffi::sg_apply_scissor_rect(x as i32, y as i32, w as i32, h as i32, true);
    }
}

/// Must be called inside a rendering pass
pub fn viewport(x: u32, y: u32, w: u32, h: u32) {
    unsafe {
        // origin_top_left: true
        ffi::sg_apply_viewport(x as i32, y as i32, w as i32, h as i32, true);
    }
}

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
        label: if label.is_empty() {
            std::ptr::null_mut()
        } else {
            // FIXME: lifetime. maybe use global CString storage?
            let label = CString::new(label).expect("Unable to create CString in BufferDesc::new");
            label.as_ptr() as *mut _
        },
        ..Default::default()
    }
}
