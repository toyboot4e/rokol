//! Draw a texture!
//!
//! FIXME: Doesn't work well with OpenGL. Why?

mod shaders;

use {
    image::{io::Reader as ImageReader, GenericImageView},
    nalgebra as na,
    rokol::{app as ra, gfx as rg},
    std::path::{Path, PathBuf},
};

fn main() -> rokol::Result {
    env_logger::init(); // give implementation to log crate

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Window".to_string(),
        ..Default::default()
    };

    let mut app = self::AppData::new();

    rokol.run(&mut app)
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
    /// X, Y, Z
    pos: [f32; 3],
    /// R, G, B, A
    color: [u8; 4],
    /// u, v (texture coordinates)
    uv: [f32; 2],
}

impl<Pos, Color, Uv> From<(Pos, Color, Uv)> for Vertex
where
    Pos: Into<[f32; 3]>,
    Color: Into<[u8; 4]>,
    Uv: Into<[f32; 2]>,
{
    fn from(data: (Pos, Color, Uv)) -> Self {
        Self {
            pos: data.0.into(),
            color: data.1.into(),
            uv: data.2.into(),
        }
    }
}

fn load_img(path: &Path) -> rg::Image {
    let img = ImageReader::open(path).unwrap().decode().unwrap();

    let (w, h) = img.dimensions();
    let pixels = img.as_bytes();

    let mut desc = rg::ImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        width: w as i32,
        height: h as i32,
        usage: rg::ResourceUsage::Immutable as u32,
        ..Default::default()
    };

    desc.content.subimage[0][0] = rg::SubimageContent {
        ptr: pixels.as_ptr() as *const _,
        size: pixels.len() as i32,
    };

    rg::make_image(&desc)
}

#[derive(Debug, Default)]
struct AppData {
    pa: rg::PassAction,
    pip: rg::Pipeline,
    bind: rg::Bindings,
}

impl AppData {
    pub fn new() -> Self {
        let color = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];

        Self {
            pa: rg::PassAction::clear(color),
            ..Default::default()
        }
    }
}

impl rokol::app::RApp for AppData {
    fn init(&mut self) {
        let mut desc = rokol::app_desc();
        rg::setup(&mut desc); // now we can call sokol_gfx functions!

        self.bind.fs_images[0] = {
            let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
            let path = root.join("examples/images/RPG Nature Tileset.png");
            self::load_img(&path)
        };

        self.bind.vertex_buffers[0] = {
            let white = [255 as u8, 255, 255, 255];

            // cube vertices
            let verts: &[Vertex] = &[
                // six rectangles
                ([-1.0, -1.0, -1.0], white, [0.0, 0.0]).into(),
                ([1.0, -1.0, -1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
                //
                ([-1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                //
                ([-1.0, -1.0, -1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([-1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                //
                ([1.0, -1.0, -1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                //
                ([1.0, -1.0, -1.0], white, [1.0, 1.0]).into(),
                ([-1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, -1.0, -1.0], white, [1.0, 1.0]).into(),
                //
                ([-1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
            ];

            let desc = rg::vtx_desc(verts, rg::ResourceUsage::Immutable, "quad-vertices");
            rg::make_buffer(&desc)
        };

        self.bind.index_buffer = {
            let indices: &[u16] = &[
                0, 1, 2, 0, 2, 3, // rectangle
                6, 5, 4, 7, 6, 4, //
                8, 9, 10, 8, 10, 11, //
                14, 13, 12, 15, 14, 12, //
                16, 17, 18, 16, 18, 19, //
                22, 21, 20, 23, 22, 20,
            ];
            let desc = &rg::idx_desc(indices, rg::ResourceUsage::Immutable, "texture-indices");
            rg::make_buffer(&desc)
        };

        self.pip = {
            let pip_desc = rg::PipelineDesc {
                shader: shaders::make_texture_shader(),
                index_type: rg::IndexType::UInt16 as u32,
                layout: rg::LayoutDesc {
                    attrs: {
                        let mut attrs = [rg::VertexAttrDesc::default(); 16];
                        attrs[0].format = rg::VertexFormat::Float3 as u32;
                        attrs[1].format = rg::VertexFormat::UByte4N as u32;
                        attrs[2].format = rg::VertexFormat::Float2 as u32;
                        attrs
                    },
                    buffers: [rg::BufferLayoutDesc::default();
                        rokol_ffi::gfx::SG_MAX_SHADERSTAGE_BUFFERS as usize],
                },
                ..Default::default()
            };

            rg::make_pipeline(&pip_desc)
        }
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(self.pip);
        rg::apply_bindings(&self.bind);

        // hmm_mat4 proj = HMM_Perspective(60.0f, (float)sapp_width()/(float)sapp_height(), 0.01f, 10.0f);
        // hmm_mat4 view = HMM_LookAt(HMM_Vec3(0.0f, 1.5f, 6.0f), HMM_Vec3(0.0f, 0.0f, 0.0f), HMM_Vec3(0.0f, 1.0f, 0.0f));
        // hmm_mat4 view_proj = HMM_MultiplyMat4(proj, view);
        // vs_params_t vs_params;
        // state.rx += 1.0f; state.ry += 2.0f;
        // hmm_mat4 rxm = HMM_Rotate(state.rx, HMM_Vec3(1.0f, 0.0f, 0.0f));
        // hmm_mat4 rym = HMM_Rotate(state.ry, HMM_Vec3(0.0f, 1.0f, 0.0f));
        // hmm_mat4 model = HMM_MultiplyMat4(rxm, rym);
        // vs_params.mvp = HMM_MultiplyMat4(view_proj, model);

        let fov = ra::width() as f32 / ra::height() as f32;
        let proj = na::Perspective3::new(60.0, fov, 0.01, 10.0);

        rg::apply_uniforms(rg::ShaderStage::Vs, 0, proj.as_matrix().as_slice());

        rg::draw(0, 36, 1);
        rg::end_pass();
        rg::commit();
    }
}
