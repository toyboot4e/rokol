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
    std::{
        ffi::{c_void, CString},
        mem::size_of,
    },
};

/// Implements [`LayoutDesc`] constructor (i.e., `layout_desc` method)
///
/// TODO: support more types?
pub use rokol_derive::LayoutDesc;

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

ffi_enum! {
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
    pub enum PassActionKind around ffi::sg_action {
        _Default = _SG_ACTION_DEFAULT,
        Clear = SG_ACTION_CLEAR,
        Load = SG_ACTION_LOAD,
        DontCare = SG_ACTION_DONTCARE,
    }
}

ffi_enum! {
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
    pub enum ResourceUsage around ffi::sg_usage {
        _Default = _SG_USAGE_DEFAULT,
        Immutable = SG_USAGE_IMMUTABLE,
        Dynamic = SG_USAGE_DYNAMIC,
        Stream = SG_USAGE_STREAM,
        _ForceU32 = _SG_USAGE_FORCE_U32,
        _Num = _SG_USAGE_NUM,
    }
}

ffi_enum! {
    /// Fs | Vs
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ShaderStage around ffi::sg_shader_stage {
        /// Fragment shader
        Fs = SG_SHADERSTAGE_FS,
        /// Vertex shader
        Vs = SG_SHADERSTAGE_VS,
    }
    // _ForceU32 = _SHADERSTAGE_FORCE_U32,
}

ffi_enum! {
    /// Mat4 | Float | Float2 | Float3 | Float4
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum UniformType around ffi::sg_uniform_type {
        Float = SG_UNIFORMTYPE_FLOAT,
        Float2 = SG_UNIFORMTYPE_FLOAT2,
        Float3 = SG_UNIFORMTYPE_FLOAT3,
        Float4 = SG_UNIFORMTYPE_FLOAT4,
        Invalid = SG_UNIFORMTYPE_INVALID,
        Mat4 = SG_UNIFORMTYPE_MAT4,
        _ForceU32 = _SG_UNIFORMTYPE_FORCE_U32,
        _Num = _SG_UNIFORMTYPE_NUM,
    }
}

ffi_enum! {
    /// Float | SInt | UInt
    ///
    /// Indicates the basic data type of a shader's texture sampler which
    /// can be float , unsigned integer or signed integer. The sampler
    /// type is used in the sg_shader_image_desc to describe the
    /// sampler type of a shader's texture sampler binding.
    ///
    /// The default sampler type is SG_SAMPLERTYPE_FLOAT.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SamplerType around ffi::sg_sampler_type {
        _Default = _SG_SAMPLERTYPE_DEFAULT,
        Float = SG_SAMPLERTYPE_FLOAT,
        SInt = SG_SAMPLERTYPE_SINT,
        UInt = SG_SAMPLERTYPE_UINT,
    }
}

// --------------------------------------------------------------------------------
// Binding enums

ffi_enum! {
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
    pub enum VertexFormat around ffi::sg_vertex_format {
        Inalid = SG_VERTEXFORMAT_INVALID,
        Float = SG_VERTEXFORMAT_FLOAT,
        Float2 = SG_VERTEXFORMAT_FLOAT2,
        Float3 = SG_VERTEXFORMAT_FLOAT3,
        Float4 = SG_VERTEXFORMAT_FLOAT4,
        Byte4 = SG_VERTEXFORMAT_BYTE4,
        Byte4N = SG_VERTEXFORMAT_BYTE4N,
        UByte4 = SG_VERTEXFORMAT_UBYTE4,
        UByte4N = SG_VERTEXFORMAT_UBYTE4N,
        Short2 = SG_VERTEXFORMAT_SHORT2,
        Short2N = SG_VERTEXFORMAT_SHORT2N,
        UShort2N = SG_VERTEXFORMAT_USHORT2N,
        Short4 = SG_VERTEXFORMAT_SHORT4,
        Short4N = SG_VERTEXFORMAT_SHORT4N,
        UShort4N = SG_VERTEXFORMAT_USHORT4N,
        Uint10N2 = SG_VERTEXFORMAT_UINT10_N2,
        _Num = _SG_VERTEXFORMAT_NUM,
        _ForceU32 = _SG_VERTEXFORMAT_FORCE_U32,
    }
}

ffi_enum! {
    /// Index | Vertex
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BufferType around ffi::sg_buffer_type {
        _Default = _SG_BUFFERTYPE_DEFAULT,
        Index = SG_BUFFERTYPE_INDEXBUFFER,
        Vertex = SG_BUFFERTYPE_VERTEXBUFFER,
        _ForceU32 = _SG_BUFFERTYPE_FORCE_U32,
        _Num = _SG_BUFFERTYPE_NUM,
    }
}

ffi_enum! {
    /// UInt16 | UInt32
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum IndexType around ffi::sg_index_type {
        _Default = _SG_INDEXTYPE_DEFAULT,
        None = SG_INDEXTYPE_NONE,
        UInt16 = SG_INDEXTYPE_UINT16,
        UInt32 = SG_INDEXTYPE_UINT32,
        _ForceU32 = _SG_INDEXTYPE_FORCE_U32,
        _Num = _SG_INDEXTYPE_NUM,
    }
}

ffi_enum! {
    /// Common subset of 3D primitive types supported across all 3D APIs. Field of [`PipelineDesc`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PrimitiveType around ffi::sg_primitive_type {
        _Default = _SG_PRIMITIVETYPE_DEFAULT,
        _ForuceU32 = _SG_PRIMITIVETYPE_FORCE_U32,
        _Num = _SG_PRIMITIVETYPE_NUM,
        Lines = SG_PRIMITIVETYPE_LINES,
        LinesStrip = SG_PRIMITIVETYPE_LINE_STRIP,
        Points = SG_PRIMITIVETYPE_POINTS,
        Triangles = SG_PRIMITIVETYPE_TRIANGLES,
        TrianglesStrip = SG_PRIMITIVETYPE_TRIANGLE_STRIP,
    }
}

// --------------------------------------------------------------------------------
// Image enums

ffi_enum! {
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
    pub enum ImageType around ffi::sg_image_type {
        _Default = _SG_IMAGETYPE_DEFAULT,
        /// 2D
        Dim2 = SG_IMAGETYPE_2D,
        /// 3D
        Dim3 = SG_IMAGETYPE_3D,
        Array = SG_IMAGETYPE_ARRAY,
        Cube = SG_IMAGETYPE_CUBE,
        _ForceU32 = _SG_IMAGETYPE_FORCE_U32,
        _Num = _SG_IMAGETYPE_NUM,
    }
}

ffi_enum! {
    /// The filtering mode when sampling a texture image
    ///
    /// This is used in the `sg_image_desc.min_filter` and `sg_image_desc.mag_filter`
    /// members when creating an image object.
    ///
    /// The default filter mode is SG_FILTER_NEAREST.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Filter around ffi::sg_filter {
        Linear = SG_FILTER_LINEAR,
        LinearMipmap = SG_FILTER_LINEAR_MIPMAP_LINEAR,
        LinearMipmapNearest = SG_FILTER_LINEAR_MIPMAP_NEAREST,
        Nearest = SG_FILTER_NEAREST,
        NearestMipmapLinear = SG_FILTER_NEAREST_MIPMAP_LINEAR,
        NearestMipmapNearest = SG_FILTER_NEAREST_MIPMAP_NEAREST,
        _Default = _SG_FILTER_DEFAULT,
        _ForceU32 = _SG_FILTER_FORCE_U32,
        _Num = _SG_FILTER_NUM,
    }
}

ffi_enum! {
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
    /// - all desktop GL platforms
    /// - Metal on macOS
    /// - D3D11
    ///
    /// Platforms which do not support clamp-to-border:
    ///
    /// - GLES2/3 and WebGL/WebGL2
    /// - Metal on iOS
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Wrap around ffi::sg_wrap {
        _Default = _SG_WRAP_DEFAULT,
        /// (Platform) Not supported on all platform
        ClampToBorder = SG_WRAP_CLAMP_TO_BORDER,
        ClampToEdge = SG_WRAP_CLAMP_TO_EDGE,
        MirroredRepeat = SG_WRAP_MIRRORED_REPEAT,
        Repeat = SG_WRAP_REPEAT,
        _ForceU32 = _SG_WRAP_FORCE_U32,
        _Wrap = _SG_WRAP_NUM,
    }
}

ffi_enum! {
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
    /// - components (R, RG, RGB or RGBA)
    /// - bit width per component (8, 16 or 32)
    /// - component data type:
    ///   - unsigned normalized (no postfix)
    ///   - signed normalized (SN postfix)
    ///   - unsigned integer (UI postfix)
    ///   - signed integer (SI postfix)
    ///   - float (F postfix)
    ///
    /// # Supported formats
    ///
    /// Not all pixel formats can be used for everything, call `sg_query_pixelformat()`
    /// to inspect the capabilities of a given pixelformat. The function returns
    /// an `sg_pixelformat_info` struct with the following bool members:
    ///
    /// - sample: the pixelformat can be sampled as texture at least with
    ///           nearest filtering
    /// - filter: the pixelformat can be samples as texture with linear
    ///           filtering
    /// - render: the pixelformat can be used for render targets
    /// - blend:  blending is supported when using the pixelformat for
    ///           render targets
    /// - msaa:   multisample-antialiasing is supported when using the
    ///           pixelformat for render targets
    /// - depth:  the pixelformat can be used for depth-stencil attachments
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
    /// - for Metal and D3D11 it is `SG_PIXELFORMAT_BGRA8`
    /// - for GL backends it is `SG_PIXELFORMAT_RGBA8`
    ///
    /// This is mainly because of the default framebuffer which is setup outside
    /// of `sokol_gfx.h`. On some backends, using BGRA for the default frame buffer
    /// allows more efficient frame flips. For your own offscreen-render-targets,
    /// use whatever renderable pixel format is convenient for you.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PixelFormat around ffi::sg_pixel_format {
        _Default = _SG_PIXELFORMAT_DEFAULT,
        Bc1Rgba = SG_PIXELFORMAT_BC1_RGBA,
        Bc2Rgba = SG_PIXELFORMAT_BC2_RGBA,
        Bc3Rgba = SG_PIXELFORMAT_BC3_RGBA,
        Bc4R = SG_PIXELFORMAT_BC4_R,
        Bc4Rsc = SG_PIXELFORMAT_BC4_RSN,
        Bc5Rg = SG_PIXELFORMAT_BC5_RG,
        Bc5Rgsn = SG_PIXELFORMAT_BC5_RGSN,
        Bc6hRgf = SG_PIXELFORMAT_BC6H_RGBF,
        Bc6hRgbuf = SG_PIXELFORMAT_BC6H_RGBUF,
        Bc7Rgba = SG_PIXELFORMAT_BC7_RGBA,
        Bgra8 = SG_PIXELFORMAT_BGRA8,
        Depth = SG_PIXELFORMAT_DEPTH,
        DepthStencil = SG_PIXELFORMAT_DEPTH_STENCIL,
        Etc2Rg11 = SG_PIXELFORMAT_ETC2_RG11,
        Etc2Rg11Sn = SG_PIXELFORMAT_ETC2_RG11SN,
        Etc2Rgb8 = SG_PIXELFORMAT_ETC2_RGB8,
        Etc2Rgb8A1 = SG_PIXELFORMAT_ETC2_RGB8A1,
        Etc2Rgba8 = SG_PIXELFORMAT_ETC2_RGBA8,
        None = SG_PIXELFORMAT_NONE,
        PvrtcRgba2Bpp = SG_PIXELFORMAT_PVRTC_RGBA_2BPP,
        PvrtcRgba24pp = SG_PIXELFORMAT_PVRTC_RGBA_4BPP,
        PvrtcRgb2Bpp = SG_PIXELFORMAT_PVRTC_RGB_2BPP,
        PvrtcRgb4Bpp = SG_PIXELFORMAT_PVRTC_RGB_4BPP,
        R8 = SG_PIXELFORMAT_R8,
        R8Si = SG_PIXELFORMAT_R8SI,
        R8Sn = SG_PIXELFORMAT_R8SN,
        R8Ui = SG_PIXELFORMAT_R8UI,
        R16 = SG_PIXELFORMAT_R16,
        R16F = SG_PIXELFORMAT_R16F,
        R16Si = SG_PIXELFORMAT_R16SI,
        R16Sn = SG_PIXELFORMAT_R16SN,
        R16Ui = SG_PIXELFORMAT_R16UI,
        R32F = SG_PIXELFORMAT_R32F,
        R32Si = SG_PIXELFORMAT_R32SI,
        R32Ui = SG_PIXELFORMAT_R32UI,
        Rg8 = SG_PIXELFORMAT_RG8,
        Rg8Si = SG_PIXELFORMAT_RG8SI,
        Rg8Sn = SG_PIXELFORMAT_RG8SN,
        Rg8Ui = SG_PIXELFORMAT_RG8UI,
        Rg11B10F = SG_PIXELFORMAT_RG11B10F,
        Rg16 = SG_PIXELFORMAT_RG16,
        Rg16F = SG_PIXELFORMAT_RG16F,
        Rg16Si = SG_PIXELFORMAT_RG16SI,
        Rg16Sn = SG_PIXELFORMAT_RG16SN,
        Rg16Ui = SG_PIXELFORMAT_RG16UI,
        Rg32F = SG_PIXELFORMAT_RG32F,
        Rg32Si = SG_PIXELFORMAT_RG32SI,
        Rg32Ui = SG_PIXELFORMAT_RG32UI,
        Rgb10A2 = SG_PIXELFORMAT_RGB10A2,
        Rgba8 = SG_PIXELFORMAT_RGBA8,
        Rgba8Si = SG_PIXELFORMAT_RGBA8SI,
        Rgba8Sn = SG_PIXELFORMAT_RGBA8SN,
        Rgba8Ui = SG_PIXELFORMAT_RGBA8UI,
        Rgba16 = SG_PIXELFORMAT_RGBA16,
        Rgba16F = SG_PIXELFORMAT_RGBA16F,
        Rgba16Si = SG_PIXELFORMAT_RGBA16SI,
        Rgba16Sn = SG_PIXELFORMAT_RGBA16SN,
        Rgba16Ui = SG_PIXELFORMAT_RGBA16UI,
        Rgba32F = SG_PIXELFORMAT_RGBA32F,
        Rgba32Si = SG_PIXELFORMAT_RGBA32SI,
        Rgba32Ui = SG_PIXELFORMAT_RGBA32UI,
        _ForceU32 = _SG_PIXELFORMAT_FORCE_U32,
        _Num = _SG_PIXELFORMAT_NUM,
    }
}

ffi_enum! {
    /// The source and destination factors in blending operations.
    ///
    /// * <https://learnopengl.com/Advanced-OpenGL/Blending>
    /// * result = src * src_factor + dst * dst_factor
    ///
    /// The default value is SG_BLENDFACTOR_ONE for source
    /// factors, and SG_BLENDFACTOR_ZERO for destination factors.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BlendFactor around ffi::sg_blend_factor {
        _Default = _SG_BLENDFACTOR_DEFAULT,
        Zero = SG_BLENDFACTOR_ZERO,
        One = SG_BLENDFACTOR_ONE,
        Color = SG_BLENDFACTOR_SRC_COLOR,
        OneMinusSourceColor = SG_BLENDFACTOR_ONE_MINUS_SRC_COLOR,
        SrcAlpha = SG_BLENDFACTOR_SRC_ALPHA,
        OneMinusSrcAlpha = SG_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
        DstColor = SG_BLENDFACTOR_DST_COLOR,
        OneMinusDstColor = SG_BLENDFACTOR_ONE_MINUS_DST_COLOR,
        DstAlpha = SG_BLENDFACTOR_DST_ALPHA,
        OneMinusDstAlpha = SG_BLENDFACTOR_ONE_MINUS_DST_ALPHA,
        SrcAlphaSatuerd = SG_BLENDFACTOR_SRC_ALPHA_SATURATED,
        BlendColor = SG_BLENDFACTOR_BLEND_COLOR,
        OneMinusBlendColor = SG_BLENDFACTOR_ONE_MINUS_BLEND_COLOR,
        BlendAlpha = SG_BLENDFACTOR_BLEND_ALPHA,
        OneMinusBlendAlpha = SG_BLENDFACTOR_ONE_MINUS_BLEND_ALPHA,
        _Num = _SG_BLENDFACTOR_NUM,
        _ForceU32 = _SG_BLENDFACTOR_FORCE_U32,
    }
}

ffi_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BlendOp around ffi::sg_blend_op {
        _Default = _SG_BLENDOP_DEFAULT,
        Add = SG_BLENDOP_ADD,
        Sub = SG_BLENDOP_SUBTRACT,
        RevSub = SG_BLENDOP_REVERSE_SUBTRACT,
        _Num = _SG_BLENDOP_NUM,
        _Force32 = _SG_BLENDOP_FORCE_U32,
    }
}

ffi_enum! {
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
    pub enum Action around ffi::sg_action {
        _Default = _SG_ACTION_DEFAULT,
        /// Clear the render target image
        Clear = SG_ACTION_CLEAR,
        /// Leave the render target image content undefined
        DontCare = SG_ACTION_DONTCARE,
        /// Load the previous content of the render target image
        Load = SG_ACTION_LOAD,
        _ForceU32 = _SG_ACTION_FORCE_U32,
        _NUM = _SG_ACTION_NUM,
    }
}

// --------------------------------------------------------------------------------
// Rendering enums

ffi_enum! {
    /// `"`, `!=`, `>`, `>=`, `<`, `<=`, `true`, `false`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompareFunc around ffi::sg_compare_func {
        _Default = _SG_COMPAREFUNC_DEFAULT,
        Never = SG_COMPAREFUNC_NEVER,
        Less = SG_COMPAREFUNC_LESS,
        Eq = SG_COMPAREFUNC_EQUAL,
        LessEq = SG_COMPAREFUNC_LESS_EQUAL,
        Greater = SG_COMPAREFUNC_GREATER,
        NotEq = SG_COMPAREFUNC_NOT_EQUAL,
        GreaterEq = SG_COMPAREFUNC_GREATER_EQUAL,
        Always = SG_COMPAREFUNC_ALWAYS,
        _Num = _SG_COMPAREFUNC_NUM,
        _ForceU32 = _SG_COMPAREFUNC_FORCE_U32,
    }
}

ffi_enum! {
    /// Front | Back | None
    ///
    /// <https://learnopengl.com/Advanced-OpenGL/Face-culling>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CullMode around ffi::sg_cull_mode {
        _Default = _SG_CULLMODE_DEFAULT,
        Back = SG_CULLMODE_BACK,
        Front = SG_CULLMODE_FRONT,
        None = SG_CULLMODE_NONE,
        _ForuceU32 = _SG_CULLMODE_FORCE_U32,
        _Num = _SG_CULLMODE_NUM,
    }
}

ffi_enum! {
    /// CCW | CW
    ///
    /// <https://learnopengl.com/Advanced-OpenGL/Face-culling>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FaceWinding around ffi::sg_face_winding {
        _Default = _SG_FACEWINDING_DEFAULT,
        /// Counter clockwise winding ordering (the default)
        Ccw = SG_FACEWINDING_CCW,
        /// Clockwise winding ordering
        Cw = SG_FACEWINDING_CW,
        _Num = _SG_FACEWINDING_NUM,
        _ForceU32 = _SG_FACEWINDING_FORCE_U32,
    }
}

bitflags::bitflags! {
    pub struct ColorMask: u32 {
        const DEFAULT = ffi::sg_color_mask::_SG_COLORMASK_DEFAULT as u32;
        const NONE = ffi::sg_color_mask::SG_COLORMASK_NONE as u32;
        const R = ffi::sg_color_mask::SG_COLORMASK_R as u32;
        const G = ffi::sg_color_mask::SG_COLORMASK_G as u32;
        const RG = ffi::sg_color_mask::SG_COLORMASK_RG as u32;
        const B = ffi::sg_color_mask::SG_COLORMASK_B as u32;
        const RB = ffi::sg_color_mask::SG_COLORMASK_RB as u32;
        const GB = ffi::sg_color_mask::SG_COLORMASK_GB as u32;
        const RGB = ffi::sg_color_mask::SG_COLORMASK_RGB as u32;
        const A = ffi::sg_color_mask::SG_COLORMASK_A as u32;
        const RA = ffi::sg_color_mask::SG_COLORMASK_RA as u32;
        const GA = ffi::sg_color_mask::SG_COLORMASK_GA as u32;
        const RGA = ffi::sg_color_mask::SG_COLORMASK_RGA as u32;
        const BA = ffi::sg_color_mask::SG_COLORMASK_BA as u32;
        const RBA = ffi::sg_color_mask::SG_COLORMASK_RBA as u32;
        const GBA = ffi::sg_color_mask::SG_COLORMASK_GBA as u32;
        const RGBA = ffi::sg_color_mask::SG_COLORMASK_RGBA as u32;
        const FORCE_U32 = ffi::sg_color_mask::_SG_COLORMASK_FORCE_U32 as u32;
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
                action: ffi::sg_action::SG_ACTION_LOAD,
                value: ffi::sg_color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            }; 4],
            depth: self::DepthAttachmentAction {
                action: ffi::sg_action::SG_ACTION_LOAD,
                value: 0.0,
            },
            stencil: self::StencilAttachmentAction {
                action: ffi::sg_action::SG_ACTION_LOAD,
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
            action: PassActionKind::Clear.to_ffi(),
            value: color.into(),
        };
        Self { raw }
    }

    pub const fn clear_const(color: [f32; 4]) -> Self {
        Self {
            raw: ffi::sg_pass_action {
                _start_canary: 0,
                colors: [self::ColorAttachmentAction {
                    action: ffi::sg_action::SG_ACTION_LOAD,
                    value: ffi::sg_color {
                        r: color[0],
                        g: color[1],
                        b: color[2],
                        a: color[3],
                    },
                }; 4],
                depth: self::DepthAttachmentAction {
                    action: ffi::sg_action::SG_ACTION_LOAD,
                    value: 0.0,
                },
                stencil: self::StencilAttachmentAction {
                    action: ffi::sg_action::SG_ACTION_LOAD,
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
        ffi::sg_apply_uniforms(stage.to_ffi(), ub_index as i32, &data);
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
        type_: buffer_type.to_ffi(),
        usage: usage.to_ffi(),
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
    use super::LayoutDesc;

    // for the derive macro:
    use crate as rokol;
    use crate::gfx as rg;

    #[derive(LayoutDesc)]
    #[repr(C)]
    pub struct Vertex {
        pub pos: [f32; 2],
        pub color: [u8; 4],
        pub uv: [f32; 2],
    }

    impl Vertex {
        pub fn manual_layout_desc() -> rg::LayoutDesc {
            let mut desc = rg::LayoutDesc::default();
            desc.attrs[0].format = rg::VertexFormat::Float2.to_ffi();
            desc.attrs[1].format = rg::VertexFormat::UByte4N.to_ffi();
            desc.attrs[2].format = rg::VertexFormat::Float2.to_ffi();
            desc
        }
    }

    #[test]
    fn layout_derive() {
        assert_eq!(Vertex::layout_desc(), Vertex::manual_layout_desc());
    }
}
