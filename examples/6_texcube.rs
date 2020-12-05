//! Draw a texture!
//!
//! FIXME: WIP

mod shaders;

use {
    glam::{Mat4, Vec3},
    image::{io::Reader as ImageReader, GenericImageView},
    rokol::{
        app as ra,
        gfx::{self as rg, BakedResource, Buffer, Image, Pipeline},
    },
    std::path::{Path, PathBuf},
};

fn main() -> rokol::Result {
    env_logger::init(); // give implementation to log crate

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Texture cube".to_string(),
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
        min_filter: rg::Filter::Linear as u32,
        mag_filter: rg::Filter::Linear as u32,
        ..Default::default()
    };

    desc.content.subimage[0][0] = rg::SubimageContent {
        ptr: pixels.as_ptr() as *const _,
        size: pixels.len() as i32,
    };

    rg::Image::create(&desc)
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
                ([1.0, -1.0, -1.0], white, [1.0, 0.0]).into(),
                ([1.0, 1.0, -1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, -1.0], white, [0.0, 1.0]).into(),
                //
                ([-1.0, -1.0, 1.0], white, [0.0, 0.0]).into(),
                ([1.0, -1.0, 1.0], white, [1.0, 0.0]).into(),
                ([1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([-1.0, 1.0, 1.0], white, [0.0, 1.0]).into(),
                //
                ([-1.0, -1.0, -1.0], white, [0.0, 0.0]).into(),
                ([-1.0, 1.0, -1.0], white, [1.0, 0.0]).into(),
                ([-1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([-1.0, -1.0, 1.0], white, [0.0, 1.0]).into(),
                //
                ([1.0, -1.0, -1.0], white, [0.0, 0.0]).into(),
                ([1.0, 1.0, -1.0], white, [1.0, 0.0]).into(),
                ([1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, -1.0, 1.0], white, [0.0, 1.0]).into(),
                //
                ([-1.0, -1.0, -1.0], white, [0.0, 0.0]).into(),
                ([-1.0, -1.0, 1.0], white, [1.0, 0.0]).into(),
                ([1.0, -1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, -1.0, -1.0], white, [0.0, 1.0]).into(),
                //
                ([-1.0, 1.0, -1.0], white, [0.0, 0.0]).into(),
                ([-1.0, 1.0, 1.0], white, [1.0, 0.0]).into(),
                ([1.0, 1.0, 1.0], white, [1.0, 1.0]).into(),
                ([1.0, 1.0, -1.0], white, [0.0, 1.0]).into(),
            ];

            let desc = rg::vbuf_desc(verts, rg::ResourceUsage::Immutable, "texcube-vertices");
            Buffer::create(&desc)
        };

        self.bind.index_buffer = {
            let indices: &[u16] = &[
                0, 1, 2, 0, 2, 3, // one rectangle
                6, 5, 4, 7, 6, 4, //
                8, 9, 10, 8, 10, 11, //
                14, 13, 12, 15, 14, 12, //
                16, 17, 18, 16, 18, 19, //
                22, 21, 20, 23, 22, 20, //
            ];
            let desc = &rg::ibuf_desc(indices, rg::ResourceUsage::Immutable, "texcube-indices");
            Buffer::create(&desc)
        };

        self.pip = Pipeline::create(&rg::PipelineDesc {
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
            shader: shaders::texcube(),
            index_type: rg::IndexType::UInt16 as u32,
            depth_stencil: rg::DepthStencilState {
                depth_compare_func: rg::CompareFunc::LessEq as u32,
                depth_write_enabled: true,
                ..Default::default()
            },
            rasterizer: rg::RasterizerState {
                cull_mode: rg::CullMode::Back as u32,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(self.pip);
        rg::apply_bindings(&self.bind);

        // left-handed matrices
        let ratio = ra::height() as f32 / ra::width() as f32;
        let proj = Mat4::perspective_lh(
            3.14 / 3.0, // fov_y_radian
            ratio,      // aspect_ratio
            0.01,       // z_near
            10.0,       // z_far
        );

        let view = Mat4::look_at_lh(
            // camera position
            [0.0, 1.5, 6.0].into(),
            // focal point
            [0.0, 0.0, 0.0].into(),
            // up direction
            [0.0, 1.0, 0.0].into(),
        );

        // column-major matrix notation (v' = Mv)
        let vp = proj * view;

        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                vp.as_ref() as *const _ as *const _,
                std::mem::size_of::<Mat4>(),
            )
        };
        rg::apply_uniforms(rg::ShaderStage::Vs, 0, bytes);

        rg::draw(0, 36, 1);

        rg::end_pass();
        rg::commit();
    }
}
