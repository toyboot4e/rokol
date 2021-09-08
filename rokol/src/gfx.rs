/*!
Graphics ([`FFI`])

[`FFI`]: rokol_ffi::gfx

# Resource types

[`BakedResource`] implementations:

* [`Buffer`]: index or vertex buffer
* [`Image`]: 2D or 3D image
* [`Pass`]: offscreen rendering pass
* [`Pipeline`]: vertex-layouts, shader and render states
* [`Shader`]: vertex and fragment shaders with shader-parameter declarations

Be sure to specify uniform names when making [`Shader`].

# Render loop

For example, for one frame with one screen rendering pass:

* [`begin_default_pass`] (screen rendering pass)
    * Optionally [`viewport`] and [`scissor`]
    * [`apply_pipeline`] (vertex-layouts, shader and render states)
    * [`apply_uniforms`] (set shader uniform block with index)
    * [`apply_bindings`] ([`Bindings`]: vertex and index buffer and images, basically a mesh)
    * [`draw`]
* [`end_pass`]
* [`commit`]

# References

* Sokol articles (The Brain Dump)
    * [A Tour of sokol_gfx.h](https://floooh.github.io/2017/07/29/sokol-gfx-tour.html) (2017)
    * [A small sokol_gfx.h API update](https://floooh.github.io/2019/01/12/sokol-apply-pipeline.html) (2019)
    * [Sokol headers: spring 2020 update](https://floooh.github.io/2020/04/26/sokol-spring-2020-update.html) (2020)
    * [Upcoming Sokol header API changes (Feb 2021)](https://floooh.github.io/2021/02/07/sokol-api-overhaul.html)
* [Learn OpenGL](https://learnopengl.com/)
* [Learn OpenGL Examples (with Sokol in C)](https://www.geertarien.com/learnopengl-examples-html5/)
* [zig-renderkit](https://github.com/prime31/zig-renderkit)
*/

use {
    rokol_ffi::gfx as ffi,
    std::ffi::{c_void, CString},
    std::mem::size_of,
};

/// Implements [`LayoutDesc`] constructor (i.e., `layout_desc` method)
///
/// TODO: support more types?
pub use rokol_derive::VertexLayout;

/// Field of [`SetupDesc`]
pub type SetupContextDesc = ffi::sg_context_desc;

/// [`setup`] parameters
pub type SetupDesc = ffi::sg_desc;

/// Sets up `sokol_gfx.h`. You'd want to use glue code in this crate.
pub fn setup(desc: &SetupDesc) {
    unsafe {
        ffi::sg_setup(desc as *const _);
    }
}

/// Cleans up `sokol_gfx.h`
pub fn shutdown() {
    unsafe {
        ffi::sg_shutdown();
    }
}

/// Can be created from `&[u8]`
///
/// Pointer-size-pair struct used to pass memory blobs into
/// sokol-gfx. When initialized from a value type (array or struct), you can
/// use the SG_RANGE() macro to build an sg_range struct. For functions which
/// take either a sg_range pointer, or a (C++) sg_range reference, use the
/// SG_RANGE_REF macro as a solution which compiles both in C and C++.
pub type Range = ffi::sg_range;

/// An RGBA color value (f32)
pub type Color = ffi::sg_color;

// --------------------------------------------------------------------------------
// Resource enums

/// Actions to be performed at the start of a rendering pass in [`begin_pass`] or [`begin_default_pass`]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PassActionKind {
    _Default = ffi::sg_action__SG_ACTION_DEFAULT as u32,
    Clear = ffi::sg_action_SG_ACTION_CLEAR as u32,
    Load = ffi::sg_action_SG_ACTION_LOAD as u32,
    DontCare = ffi::sg_action_SG_ACTION_DONTCARE as u32,
}

/// Update strategy of buffers and images
///
/// Render target image has to have `Immutable` usage.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ResourceUsage {
    _Default = ffi::sg_usage__SG_USAGE_DEFAULT as u32,
    Immutable = ffi::sg_usage_SG_USAGE_IMMUTABLE as u32,
    Dynamic = ffi::sg_usage_SG_USAGE_DYNAMIC as u32,
    Stream = ffi::sg_usage_SG_USAGE_STREAM as u32,
    _ForceU32 = ffi::sg_usage__SG_USAGE_FORCE_U32 as u32,
    _Num = ffi::sg_usage__SG_USAGE_NUM as u32,
}

/// Fs | Vs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ShaderStage {
    /// Fragment shader
    Fs = ffi::sg_shader_stage_SG_SHADERSTAGE_FS as u32,
    /// Vertex shader
    Vs = ffi::sg_shader_stage_SG_SHADERSTAGE_VS as u32,
    // _ForceU32 = ffi::sg_shader_stage__SG_SHADERSTAGE_FORCE_U32 as u32,
}

/// Mat4 | Float | Float2 | Float3 | Float4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UniformType {
    Float = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT as u32,
    Float2 = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT2 as u32,
    Float3 = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT3 as u32,
    Float4 = ffi::sg_uniform_type_SG_UNIFORMTYPE_FLOAT4 as u32,
    Invalid = ffi::sg_uniform_type_SG_UNIFORMTYPE_INVALID as u32,
    Mat4 = ffi::sg_uniform_type_SG_UNIFORMTYPE_MAT4 as u32,
    _ForceU32 = ffi::sg_uniform_type__SG_UNIFORMTYPE_FORCE_U32 as u32,
    _Num = ffi::sg_uniform_type__SG_UNIFORMTYPE_NUM as u32,
}

/// Float | SInt | UInt
///
/// Indicates the basic data type of a shader's texture sampler which
/// can be float , unsigned integer or signed integer. The sampler
/// type is used in the sg_shader_image_desc to describe the
/// sampler type of a shader's texture sampler binding.
///
/// The default sampler type is SG_SAMPLERTYPE_FLOAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SamplerType {
    _Default = ffi::sg_sampler_type__SG_SAMPLERTYPE_DEFAULT as u32,
    Float = ffi::sg_sampler_type_SG_SAMPLERTYPE_FLOAT as u32,
    SInt = ffi::sg_sampler_type_SG_SAMPLERTYPE_SINT as u32,
    UInt = ffi::sg_sampler_type_SG_SAMPLERTYPE_UINT as u32,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VertexFormat {
    Inalid = ffi::sg_vertex_format_SG_VERTEXFORMAT_INVALID as u32,
    Float = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT as u32,
    Float2 = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT2 as u32,
    Float3 = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT3 as u32,
    Float4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_FLOAT4 as u32,
    Byte4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_BYTE4 as u32,
    Byte4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_BYTE4N as u32,
    UByte4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_UBYTE4 as u32,
    UByte4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_UBYTE4N as u32,
    Short2 = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT2 as u32,
    Short2N = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT2N as u32,
    UShort2N = ffi::sg_vertex_format_SG_VERTEXFORMAT_USHORT2N as u32,
    Short4 = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT4 as u32,
    Short4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_SHORT4N as u32,
    UShort4N = ffi::sg_vertex_format_SG_VERTEXFORMAT_USHORT4N as u32,
    Uint10N2 = ffi::sg_vertex_format_SG_VERTEXFORMAT_UINT10_N2 as u32,
    _Num = ffi::sg_vertex_format__SG_VERTEXFORMAT_NUM as u32,
    _ForceU32 = ffi::sg_vertex_format__SG_VERTEXFORMAT_FORCE_U32 as u32,
}

/// Index | Vertex
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BufferType {
    _Default = ffi::sg_buffer_type__SG_BUFFERTYPE_DEFAULT as u32,
    Index = ffi::sg_buffer_type_SG_BUFFERTYPE_INDEXBUFFER as u32,
    Vertex = ffi::sg_buffer_type_SG_BUFFERTYPE_VERTEXBUFFER as u32,
    _ForceU32 = ffi::sg_buffer_type__SG_BUFFERTYPE_FORCE_U32 as u32,
    _Num = ffi::sg_buffer_type__SG_BUFFERTYPE_NUM as u32,
}

/// UInt16 | UInt32
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IndexType {
    _Default = ffi::sg_index_type__SG_INDEXTYPE_DEFAULT as u32,
    None = ffi::sg_index_type_SG_INDEXTYPE_NONE as u32,
    UInt16 = ffi::sg_index_type_SG_INDEXTYPE_UINT16 as u32,
    UInt32 = ffi::sg_index_type_SG_INDEXTYPE_UINT32 as u32,
    _ForceU32 = ffi::sg_index_type__SG_INDEXTYPE_FORCE_U32 as u32,
    _Num = ffi::sg_index_type__SG_INDEXTYPE_NUM as u32,
}

/// Common subset of 3D primitive types supported across all 3D APIs. Field of [`PipelineDesc`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PrimitiveType {
    _Default = ffi::sg_primitive_type__SG_PRIMITIVETYPE_DEFAULT as u32,
    _ForuceU32 = ffi::sg_primitive_type__SG_PRIMITIVETYPE_FORCE_U32 as u32,
    _Num = ffi::sg_primitive_type__SG_PRIMITIVETYPE_NUM as u32,
    Lines = ffi::sg_primitive_type_SG_PRIMITIVETYPE_LINES as u32,
    LinesStrip = ffi::sg_primitive_type_SG_PRIMITIVETYPE_LINE_STRIP as u32,
    Points = ffi::sg_primitive_type_SG_PRIMITIVETYPE_POINTS as u32,
    Triangles = ffi::sg_primitive_type_SG_PRIMITIVETYPE_TRIANGLES as u32,
    TrianglesStrip = ffi::sg_primitive_type_SG_PRIMITIVETYPE_TRIANGLE_STRIP as u32,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ImageType {
    _Default = ffi::sg_image_type__SG_IMAGETYPE_DEFAULT as u32,
    /// 2D
    Dim2 = ffi::sg_image_type_SG_IMAGETYPE_2D as u32,
    /// 3D
    Dim3 = ffi::sg_image_type_SG_IMAGETYPE_3D as u32,
    Array = ffi::sg_image_type_SG_IMAGETYPE_ARRAY as u32,
    Cube = ffi::sg_image_type_SG_IMAGETYPE_CUBE as u32,
    _ForceU32 = ffi::sg_image_type__SG_IMAGETYPE_FORCE_U32 as u32,
    _Num = ffi::sg_image_type__SG_IMAGETYPE_NUM as u32,
}

/// The filtering mode when sampling a texture image
///
/// This is used in the `sg_image_desc.min_filter` and `sg_image_desc.mag_filter`
/// members when creating an image object.
///
/// The default filter mode is SG_FILTER_NEAREST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Filter {
    Linear = ffi::sg_filter_SG_FILTER_LINEAR as u32,
    LinearMipmap = ffi::sg_filter_SG_FILTER_LINEAR_MIPMAP_LINEAR as u32,
    LinearMipmapNearest = ffi::sg_filter_SG_FILTER_LINEAR_MIPMAP_NEAREST as u32,
    Nearest = ffi::sg_filter_SG_FILTER_NEAREST as u32,
    NearestMipmapLinear = ffi::sg_filter_SG_FILTER_NEAREST_MIPMAP_LINEAR as u32,
    NearestMipmapNearest = ffi::sg_filter_SG_FILTER_NEAREST_MIPMAP_NEAREST as u32,
    _Default = ffi::sg_filter__SG_FILTER_DEFAULT as u32,
    _ForceU32 = ffi::sg_filter__SG_FILTER_FORCE_U32 as u32,
    _Num = ffi::sg_filter__SG_FILTER_NUM as u32,
}

/// The texture coordinates wrapping mode when sampling a texture image
///
/// This is used in [`rokol_ffi::gfx::sg_image_desc`] when creating an image..
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Wrap {
    _Default = ffi::sg_wrap__SG_WRAP_DEFAULT as u32,
    /// (Platform) Not supported on all platform
    ClampToBorder = ffi::sg_wrap_SG_WRAP_CLAMP_TO_BORDER as u32,
    ClampToEdge = ffi::sg_wrap_SG_WRAP_CLAMP_TO_EDGE as u32,
    MirroredRepeat = ffi::sg_wrap_SG_WRAP_MIRRORED_REPEAT as u32,
    Repeat = ffi::sg_wrap_SG_WRAP_REPEAT as u32,
    _ForceU32 = ffi::sg_wrap__SG_WRAP_FORCE_U32 as u32,
    _Wrap = ffi::sg_wrap__SG_WRAP_NUM as u32,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PixelFormat {
    _Default = ffi::sg_pixel_format__SG_PIXELFORMAT_DEFAULT as u32 as u32,
    Bc1Rgba = ffi::sg_pixel_format_SG_PIXELFORMAT_BC1_RGBA as u32 as u32,
    Bc2Rgba = ffi::sg_pixel_format_SG_PIXELFORMAT_BC2_RGBA as u32 as u32,
    Bc3Rgba = ffi::sg_pixel_format_SG_PIXELFORMAT_BC3_RGBA as u32 as u32,
    Bc4R = ffi::sg_pixel_format_SG_PIXELFORMAT_BC4_R as u32 as u32,
    Bc4Rsc = ffi::sg_pixel_format_SG_PIXELFORMAT_BC4_RSN as u32 as u32,
    Bc5Rg = ffi::sg_pixel_format_SG_PIXELFORMAT_BC5_RG as u32 as u32,
    Bc5Rgsn = ffi::sg_pixel_format_SG_PIXELFORMAT_BC5_RGSN as u32 as u32,
    Bc6hRgf = ffi::sg_pixel_format_SG_PIXELFORMAT_BC6H_RGBF as u32 as u32,
    Bc6hRgbuf = ffi::sg_pixel_format_SG_PIXELFORMAT_BC6H_RGBUF as u32 as u32,
    Bc7Rgba = ffi::sg_pixel_format_SG_PIXELFORMAT_BC7_RGBA as u32 as u32,
    Bgra8 = ffi::sg_pixel_format_SG_PIXELFORMAT_BGRA8 as u32 as u32,
    Depth = ffi::sg_pixel_format_SG_PIXELFORMAT_DEPTH as u32 as u32,
    DepthStencil = ffi::sg_pixel_format_SG_PIXELFORMAT_DEPTH_STENCIL as u32 as u32,
    Etc2Rg11 = ffi::sg_pixel_format_SG_PIXELFORMAT_ETC2_RG11 as u32 as u32,
    Etc2Rg11Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_ETC2_RG11SN as u32 as u32,
    Etc2Rgb8 = ffi::sg_pixel_format_SG_PIXELFORMAT_ETC2_RGB8 as u32 as u32,
    Etc2Rgb8A1 = ffi::sg_pixel_format_SG_PIXELFORMAT_ETC2_RGB8A1 as u32 as u32,
    Etc2Rgba8 = ffi::sg_pixel_format_SG_PIXELFORMAT_ETC2_RGBA8 as u32 as u32,
    None = ffi::sg_pixel_format_SG_PIXELFORMAT_NONE as u32 as u32,
    PvrtcRgba2Bpp = ffi::sg_pixel_format_SG_PIXELFORMAT_PVRTC_RGBA_2BPP as u32 as u32,
    PvrtcRgba24pp = ffi::sg_pixel_format_SG_PIXELFORMAT_PVRTC_RGBA_4BPP as u32 as u32,
    PvrtcRgb2Bpp = ffi::sg_pixel_format_SG_PIXELFORMAT_PVRTC_RGB_2BPP as u32 as u32,
    PvrtcRgb4Bpp = ffi::sg_pixel_format_SG_PIXELFORMAT_PVRTC_RGB_4BPP as u32 as u32,
    R8 = ffi::sg_pixel_format_SG_PIXELFORMAT_R8 as u32 as u32,
    R8Si = ffi::sg_pixel_format_SG_PIXELFORMAT_R8SI as u32 as u32,
    R8Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_R8SN as u32 as u32,
    R8Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_R8UI as u32 as u32,
    R16 = ffi::sg_pixel_format_SG_PIXELFORMAT_R16 as u32 as u32,
    R16F = ffi::sg_pixel_format_SG_PIXELFORMAT_R16F as u32 as u32,
    R16Si = ffi::sg_pixel_format_SG_PIXELFORMAT_R16SI as u32 as u32,
    R16Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_R16SN as u32 as u32,
    R16Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_R16UI as u32 as u32,
    R32F = ffi::sg_pixel_format_SG_PIXELFORMAT_R32F as u32 as u32,
    R32Si = ffi::sg_pixel_format_SG_PIXELFORMAT_R32SI as u32 as u32,
    R32Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_R32UI as u32 as u32,
    Rg8 = ffi::sg_pixel_format_SG_PIXELFORMAT_RG8 as u32 as u32,
    Rg8Si = ffi::sg_pixel_format_SG_PIXELFORMAT_RG8SI as u32 as u32,
    Rg8Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_RG8SN as u32 as u32,
    Rg8Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_RG8UI as u32 as u32,
    Rg11B10F = ffi::sg_pixel_format_SG_PIXELFORMAT_RG11B10F as u32 as u32,
    Rg16 = ffi::sg_pixel_format_SG_PIXELFORMAT_RG16 as u32 as u32,
    Rg16F = ffi::sg_pixel_format_SG_PIXELFORMAT_RG16F as u32 as u32,
    Rg16Si = ffi::sg_pixel_format_SG_PIXELFORMAT_RG16SI as u32 as u32,
    Rg16Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_RG16SN as u32 as u32,
    Rg16Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_RG16UI as u32 as u32,
    Rg32F = ffi::sg_pixel_format_SG_PIXELFORMAT_RG32F as u32 as u32,
    Rg32Si = ffi::sg_pixel_format_SG_PIXELFORMAT_RG32SI as u32 as u32,
    Rg32Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_RG32UI as u32 as u32,
    Rgb10A2 = ffi::sg_pixel_format_SG_PIXELFORMAT_RGB10A2 as u32 as u32,
    Rgba8 = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA8 as u32 as u32,
    Rgba8Si = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA8SI as u32 as u32,
    Rgba8Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA8SN as u32 as u32,
    Rgba8Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA8UI as u32 as u32,
    Rgba16 = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA16 as u32 as u32,
    Rgba16F = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA16F as u32 as u32,
    Rgba16Si = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA16SI as u32 as u32,
    Rgba16Sn = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA16SN as u32 as u32,
    Rgba16Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA16UI as u32 as u32,
    Rgba32F = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA32F as u32 as u32,
    Rgba32Si = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA32SI as u32 as u32,
    Rgba32Ui = ffi::sg_pixel_format_SG_PIXELFORMAT_RGBA32UI as u32 as u32,
    _ForceU32 = ffi::sg_pixel_format__SG_PIXELFORMAT_FORCE_U32 as u32 as u32,
    _Num = ffi::sg_pixel_format__SG_PIXELFORMAT_NUM as u32 as u32,
}

/// The source and destination factors in blending operations.
///
/// * <https://learnopengl.com/Advanced-OpenGL/Blending>
/// * result = src * src_factor + dst * dst_factor
///
/// The default value is SG_BLENDFACTOR_ONE for source
/// factors, and SG_BLENDFACTOR_ZERO for destination factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BlendFactor {
    _Default = ffi::sg_blend_factor__SG_BLENDFACTOR_DEFAULT as u32 as u32,
    Zero = ffi::sg_blend_factor_SG_BLENDFACTOR_ZERO as u32 as u32,
    One = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE as u32 as u32,
    Color = ffi::sg_blend_factor_SG_BLENDFACTOR_SRC_COLOR as u32 as u32,
    OneMinusSourceColor = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE_MINUS_SRC_COLOR as u32 as u32,
    SrcAlpha = ffi::sg_blend_factor_SG_BLENDFACTOR_SRC_ALPHA as u32 as u32,
    OneMinusSrcAlpha = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE_MINUS_SRC_ALPHA as u32 as u32,
    DstColor = ffi::sg_blend_factor_SG_BLENDFACTOR_DST_COLOR as u32 as u32,
    OneMinusDstColor = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE_MINUS_DST_COLOR as u32 as u32,
    DstAlpha = ffi::sg_blend_factor_SG_BLENDFACTOR_DST_ALPHA as u32 as u32,
    OneMinusDstAlpha = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE_MINUS_DST_ALPHA as u32 as u32,
    SrcAlphaSatuerd = ffi::sg_blend_factor_SG_BLENDFACTOR_SRC_ALPHA_SATURATED as u32 as u32,
    BlendColor = ffi::sg_blend_factor_SG_BLENDFACTOR_BLEND_COLOR as u32 as u32,
    OneMinusBlendColor = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE_MINUS_BLEND_COLOR as u32 as u32,
    BlendAlpha = ffi::sg_blend_factor_SG_BLENDFACTOR_BLEND_ALPHA as u32 as u32,
    OneMinusBlendAlpha = ffi::sg_blend_factor_SG_BLENDFACTOR_ONE_MINUS_BLEND_ALPHA as u32 as u32,
    _Num = ffi::sg_blend_factor__SG_BLENDFACTOR_NUM as u32 as u32,
    _ForceU32 = ffi::sg_blend_factor__SG_BLENDFACTOR_FORCE_U32 as u32 as u32,
}

/// Defines what action should be performed at the start of a render pass:
///
/// This is used in the [`PassAction`] structure.
///
/// The default action for all pass attachments is `Clear`, with the
/// clear color rgba = {0.5f, 0.5f, 0.5f, 1.0f], depth=1.0 and stencil=0.
///
/// If you want to override the default behaviour, it is important to not
/// only set the clear color, but the 'action' field as well (as long as this
/// is in its _SG_ACTION_DEFAULT, the value fields will be ignored).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Action {
    _Default = ffi::sg_action__SG_ACTION_DEFAULT as u32 as u32,
    /// Clear the render target image
    Clear = ffi::sg_action_SG_ACTION_CLEAR as u32 as u32,
    /// Leave the render target image content undefined
    DontCare = ffi::sg_action_SG_ACTION_DONTCARE as u32 as u32,
    /// Load the previous content of the render target image
    Load = ffi::sg_action_SG_ACTION_LOAD as u32 as u32,
    _ForceU32 = ffi::sg_action__SG_ACTION_FORCE_U32 as u32 as u32,
    _NUM = ffi::sg_action__SG_ACTION_NUM as u32 as u32,
}

// --------------------------------------------------------------------------------
// Rendering enums

/// `"`, `!=`, `>`, `>=`, `<`, `<=`, `true`, `false`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CompareFunc {
    _Default = ffi::sg_compare_func__SG_COMPAREFUNC_DEFAULT as u32,
    Always = ffi::sg_compare_func_SG_COMPAREFUNC_ALWAYS as u32,
    Eq = ffi::sg_compare_func_SG_COMPAREFUNC_EQUAL as u32,
    Greater = ffi::sg_compare_func_SG_COMPAREFUNC_GREATER as u32,
    GreaterEq = ffi::sg_compare_func_SG_COMPAREFUNC_GREATER_EQUAL as u32,
    Less = ffi::sg_compare_func_SG_COMPAREFUNC_LESS as u32,
    LessEq = ffi::sg_compare_func_SG_COMPAREFUNC_LESS_EQUAL as u32,
    Never = ffi::sg_compare_func_SG_COMPAREFUNC_NEVER as u32,
    NotEq = ffi::sg_compare_func_SG_COMPAREFUNC_NOT_EQUAL as u32,
    _ForceU32 = ffi::sg_compare_func__SG_COMPAREFUNC_FORCE_U32 as u32,
    _Num = ffi::sg_compare_func__SG_COMPAREFUNC_NUM as u32,
}

/// Front | Back | None
///
/// <https://learnopengl.com/Advanced-OpenGL/Face-culling>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CullMode {
    _Default = ffi::sg_cull_mode__SG_CULLMODE_DEFAULT as u32,
    Back = ffi::sg_cull_mode_SG_CULLMODE_BACK as u32,
    Front = ffi::sg_cull_mode_SG_CULLMODE_FRONT as u32,
    None = ffi::sg_cull_mode_SG_CULLMODE_NONE as u32,
    _ForuceU32 = ffi::sg_cull_mode__SG_CULLMODE_FORCE_U32 as u32,
    _Num = ffi::sg_cull_mode__SG_CULLMODE_NUM as u32,
}

/// CCW | CW
///
/// <https://learnopengl.com/Advanced-OpenGL/Face-culling>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FaceWinding {
    _Default = ffi::sg_face_winding__SG_FACEWINDING_DEFAULT as u32,
    /// Counter clockwise winding ordering (the default)
    Ccw = ffi::sg_face_winding_SG_FACEWINDING_CCW as u32,
    /// Clockwise winding ordering
    Cw = ffi::sg_face_winding_SG_FACEWINDING_CW as u32,
    _Num = ffi::sg_face_winding__SG_FACEWINDING_NUM as u32,
    _ForceU32 = ffi::sg_face_winding__SG_FACEWINDING_FORCE_U32 as u32,
}

bitflags::bitflags! {
    pub struct ColorMask: u32 {
        const DEFAULT = ffi::sg_color_mask__SG_COLORMASK_DEFAULT as u32;
        const NONE = ffi::sg_color_mask_SG_COLORMASK_NONE as u32;
        const R = ffi::sg_color_mask_SG_COLORMASK_R as u32;
        const G = ffi::sg_color_mask_SG_COLORMASK_G as u32;
        const RG = ffi::sg_color_mask_SG_COLORMASK_RG as u32;
        const B = ffi::sg_color_mask_SG_COLORMASK_B as u32;
        const RB = ffi::sg_color_mask_SG_COLORMASK_RB as u32;
        const GB = ffi::sg_color_mask_SG_COLORMASK_GB as u32;
        const RGB = ffi::sg_color_mask_SG_COLORMASK_RGB as u32;
        const A = ffi::sg_color_mask_SG_COLORMASK_A as u32;
        const RA = ffi::sg_color_mask_SG_COLORMASK_RA as u32;
        const GA = ffi::sg_color_mask_SG_COLORMASK_GA as u32;
        const RGA = ffi::sg_color_mask_SG_COLORMASK_RGA as u32;
        const BA = ffi::sg_color_mask_SG_COLORMASK_BA as u32;
        const RBA = ffi::sg_color_mask_SG_COLORMASK_RBA as u32;
        const GBA = ffi::sg_color_mask_SG_COLORMASK_GBA as u32;
        const RGBA = ffi::sg_color_mask_SG_COLORMASK_RGBA as u32;
        const FORCE_U32 = ffi::sg_color_mask__SG_COLORMASK_FORCE_U32 as u32;
    }
}

/// Pass action
///
/// Internally, it just wraps [`ffi::sg_pass_action`] to add methods without using a trait.
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

    /// Untouch (load) the last content
    pub const LOAD: Self = Self {
        raw: ffi::sg_pass_action {
            _start_canary: 0,
            colors: [self::ColorAttachmentAction {
                action: self::Action::Load as u32,
                value: ffi::sg_color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            }; 4],
            depth: self::DepthAttachmentAction {
                action: self::Action::Load as u32,
                value: 0.0,
            },
            stencil: self::StencilAttachmentAction {
                action: self::Action::Load as u32,
                value: 0,
            },
            _end_canary: 0,
        },
    };

    pub fn new(raw: ffi::sg_pass_action) -> Self {
        Self { raw }
    }

    pub const fn new_const(raw: ffi::sg_pass_action) -> Self {
        Self { raw }
    }

    pub fn clear(color: impl Into<Color>) -> Self {
        let mut raw = ffi::sg_pass_action::default();
        raw.colors[0] = ColorAttachmentAction {
            action: PassActionKind::Clear as u32,
            value: color.into(),
        };
        Self { raw }
    }

    pub const fn clear_const(color: [f32; 4]) -> Self {
        Self {
            raw: ffi::sg_pass_action {
                _start_canary: 0,
                colors: [self::ColorAttachmentAction {
                    action: self::Action::Load as u32,
                    value: ffi::sg_color {
                        r: color[0],
                        g: color[1],
                        b: color[2],
                        a: color[3],
                    },
                }; 4],
                depth: self::DepthAttachmentAction {
                    action: self::Action::Load as u32,
                    value: 0.0,
                },
                stencil: self::StencilAttachmentAction {
                    action: self::Action::Load as u32,
                    value: 0,
                },
                _end_canary: 0,
            },
        }
    }

    // TODO: add more constructors
}

// --------------------------------------------------------------------------------
// Re-exports from the FFI

pub type PassAttachmentDesc = ffi::sg_pass_attachment_desc;

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
/// The width and height are scaled size (e.g. if on 2x DPI monitor display with 2560x1440 pixels,
/// give scaled size of 1280x720.
pub type ImageDesc = ffi::sg_image_desc;
pub type ImageData = ffi::sg_image_data;
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
pub type DepthState = ffi::sg_depth_state;

pub type Features = ffi::sg_features;

pub type LayoutDesc = ffi::sg_layout_desc;
pub type Limits = ffi::sg_limits;

pub type PixelFormatInfo = ffi::sg_pixelformat_info;

pub type ShaderUniformBlockDesc = ffi::sg_shader_uniform_block_desc;
pub type ShaderUniformDesc = ffi::sg_shader_uniform_desc;

pub type SlotInfo = ffi::sg_slot_info;
pub type StencilAttachmentAction = ffi::sg_stencil_attachment_action;
pub type StencilState = ffi::sg_stencil_state;

pub type TraceHooks = ffi::sg_trace_hooks;
pub type VertexAttrDesc = ffi::sg_vertex_attr_desc;

pub type GlContextDesc = ffi::sg_gl_context_desc;
pub type D3D11ContextDesc = ffi::sg_d3d11_context_desc;
pub type MetalContextDesc = ffi::sg_metal_context_desc;
pub type WgpuContextDesc = ffi::sg_wgpu_context_desc;

// --------------------------------------------------------------------------------
// Functions

/// Screen rendering pass. Pass framebuffer size as arguments
pub fn begin_default_pass(pa: &impl AsRef<ffi::sg_pass_action>, w: u32, h: u32) {
    unsafe {
        ffi::sg_begin_default_pass(pa.as_ref(), w as i32, h as i32);
    }
}

/// Screen rendering pass. Pass framebuffer size as arguments
pub fn begin_default_pass_f(pa: &impl AsRef<ffi::sg_pass_action>, w: f32, h: f32) {
    unsafe {
        ffi::sg_begin_default_passf(pa.as_ref(), w, h);
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
pub fn apply_uniforms(stage: ShaderStage, ub_index: u32, data: &[u8]) {
    let data = Range {
        ptr: data.as_ptr() as *mut _,
        size: (size_of::<u8>() * data.len()) as _,
    };
    unsafe {
        ffi::sg_apply_uniforms(stage as u32, ub_index as i32, &data);
    }
}

/// `draw(base_elems, n_elems, n_instances)`
pub fn draw(base_elem: u32, n_elems: u32, n_instances: u32) {
    unsafe {
        ffi::sg_draw(base_elem as i32, n_elems as i32, n_instances as i32);
    }
}

/// Discard output fragments outside of this rectangle
///
/// Must be called inside a rendering pass
///
/// The (0, 0) point is at the left-bottom corner of the target. TODO: really?
pub fn scissor(x: u32, y: u32, w: u32, h: u32) {
    unsafe {
        // origin_top_left: true
        ffi::sg_apply_scissor_rect(x as i32, y as i32, w as i32, h as i32, true);
    }
}

pub fn scissor_f(x: f32, y: f32, w: f32, h: f32) {
    unsafe {
        // origin_top_left: true
        ffi::sg_apply_scissor_rectf(x, y, w, h, true);
    }
}

/// Output rectangle space
///
/// Must be called inside a rendering pass
///
/// The (0, 0) point is at the left-bottom corner of the target. TODO: really?
pub fn viewport(x: u32, y: u32, w: u32, h: u32) {
    unsafe {
        // origin_top_left: true
        ffi::sg_apply_viewport(x as i32, y as i32, w as i32, h as i32, true);
    }
}

pub fn viewport_f(x: f32, y: f32, w: f32, h: f32) {
    unsafe {
        // origin_top_left: true
        ffi::sg_apply_viewportf(x, y, w, h, true);
    }
}
/// Uploads vertices/indices to vertex/index buffer
///
/// Requires [`ResourceUsage::Dynamic`] or [`ResourceUsage::Stream`].
///
/// WARNING: can be called only once a frame
pub unsafe fn update_buffer(buf: Buffer, data: &[u8]) {
    let size = size_of::<u8>() * data.len();
    let data = Range {
        ptr: data.as_ptr() as *const _,
        size: size as _,
    };
    ffi::sg_update_buffer(buf, &data);
}

/// Appends vertices/indices to vertex/index buffer
///
/// Requires [`ResourceUsage::Dynamic`] or [`ResourceUsage::Stream`]. This can be called multiple
/// times per frame.
///
/// Returns a byte offset to the start of the written data. The offset can be assgined to
/// [`Bindings::vertex_buffer_offsets`] or [`Bindings::index_buffer_offset`].
pub fn append_buffer(buf: Buffer, data: &[u8]) -> i32 {
    let n_bytes = size_of::<u8>() * data.len();
    let data = Range {
        ptr: data.as_ptr() as *const _,
        size: n_bytes as _,
    };
    unsafe { ffi::sg_append_buffer(buf, &data) }
}

/// Only one update per frame is allowed for buffer and image resources
pub unsafe fn update_image(img: Image, content: &ImageData) {
    ffi::sg_update_image(img, content);
}

// --------------------------------------------------------------------------------
// Helper functions

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
pub fn ibuf_desc_immutable(buf: &[u8], label: &str) -> BufferDesc {
    let size = std::mem::size_of::<u8>() * buf.len();
    unsafe {
        buf_desc(
            buf.as_ptr() as *const _,
            size,
            BufferType::Index,
            ResourceUsage::Immutable,
            label,
        )
    }
}

/// [Non-Sokol] Helper for creating index buffer
pub fn ibuf_desc_dyn(size: usize, usage: ResourceUsage, label: &str) -> BufferDesc {
    unsafe { buf_desc(std::ptr::null_mut(), size, BufferType::Index, usage, label) }
}

/// [Non-Sokol] Helper for creating immutable vertex buffer
pub fn vbuf_desc_immutable(buf: &[u8], label: &str) -> BufferDesc {
    let size = std::mem::size_of::<u8>() * buf.len();
    unsafe {
        buf_desc(
            buf.as_ptr() as *const _,
            size,
            BufferType::Vertex,
            ResourceUsage::Immutable,
            label,
        )
    }
}

/// [Non-Sokol] Helper for creating dynamic vertex buffer
pub fn vbuf_desc_dyn(size: usize, usage: ResourceUsage, label: &str) -> BufferDesc {
    unsafe { buf_desc(std::ptr::null_mut(), size, BufferType::Vertex, usage, label) }
}

/// [Non-Sokol] Helper for creating dynamic vertex buffer
pub unsafe fn buf_desc(
    // null if dyn, some if immutable
    data_ptr: *const c_void,
    data_size: usize,
    buffer_type: BufferType,
    usage: ResourceUsage,
    label: &str,
) -> BufferDesc {
    ffi::sg_buffer_desc {
        size: data_size as _,
        data: Range {
            ptr: data_ptr,
            size: data_size as _,
        },
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

#[cfg(test)]
mod test {
    use crate::gfx::{self as rg, VertexLayout};

    // for the derive macro:
    use crate as rokol;

    #[derive(VertexLayout)]
    #[repr(C)]
    pub struct Vertex {
        pub pos: [f32; 2],
        pub color: [u8; 4],
        pub uv: [f32; 2],
    }

    impl Vertex {
        pub fn manual_layout_desc() -> rg::LayoutDesc {
            let mut desc = rg::LayoutDesc::default();
            desc.attrs[0].format = rg::VertexFormat::Float2 as u32;
            desc.attrs[1].format = rg::VertexFormat::UByte4N as u32;
            desc.attrs[2].format = rg::VertexFormat::Float2 as u32;
            desc
        }
    }

    #[test]
    fn layout() {
        assert_eq!(Vertex::layout_desc(), Vertex::manual_layout_desc());
    }
}
