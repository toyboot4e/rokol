//! `rokol::gfx`, graphics

use {
    bitflags::bitflags,
    rokol_ffi::gfx as ffi,
    std::{
        ffi::{c_void, CStr, CString},
        os::raw::{c_char, c_int},
    },
};

pub fn setup(desc: &mut Desc) {
    unsafe {
        ffi::sg_setup(desc as *const _ as *mut _);
    }
}

// --------------------------------------------------------------------------------
// Enums

/// Actions to be performed at the start of a rendering pass in `begin_pass` or `begindefault_pass`
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
pub enum RgAction {
    // __Default = ffi::sg_action_SG_ACTION__DEFAULT,
    Clear = ffi::sg_action_SG_ACTION_CLEAR,
    Load = ffi::sg_action_SG_ACTION_LOAD,
    DontCare = ffi::sg_action_SG_ACTION_DONTCARE,
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
            action: RgAction::Clear as u32,
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
pub type Buffer = ffi::sg_buffer;
pub type BufferDesc = ffi::sg_buffer_desc;
pub type BufferInfo = ffi::sg_buffer_info;
pub type BufferLayoutDesc = ffi::sg_buffer_layout_desc;
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
pub type Pipeline = ffi::sg_pipeline;
pub type PipelineDesc = ffi::sg_pipeline_desc;
pub type PipelineInfo = ffi::sg_pipeline_info;
pub type PixelFormatInfo = ffi::sg_pixelformat_info;

pub type Shader = ffi::sg_shader;
pub type ShaderAttrDesc = ffi::sg_shader_attr_desc;
pub type ShaderDesc = ffi::sg_shader_desc;
pub type ShaderImageDesc = ffi::sg_shader_image_desc;
pub type ShaderInfo = ffi::sg_shader_info;
pub type ShaderStateDesc = ffi::sg_shader_stage_desc;
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
