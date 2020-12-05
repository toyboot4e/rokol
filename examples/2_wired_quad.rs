//! Draw a quadliteral!

mod shaders;

use rokol::{
    app as ra,
    gfx::{self as rg, BakedResource, Buffer, Pipeline},
};

fn main() -> rokol::Result {
    env_logger::init(); // give implementation to log crate

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Quad wireframe (lines)".to_string(),
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
    color: [f32; 4],
}

impl<T, U> From<(T, U)> for Vertex
where
    T: Into<[f32; 3]>,
    U: Into<[f32; 4]>,
{
    fn from(data: (T, U)) -> Self {
        Self {
            pos: data.0.into(),
            color: data.1.into(),
        }
    }
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
        rg::setup(&mut rokol::app_desc());
        // now we can call `sokol_gfx` functions!

        self.bind.vertex_buffers[0] = {
            let verts: &[Vertex] = &[
                // top left
                ([-0.5, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]).into(),
                // top right
                ([0.5, 0.5, 0.5], [0.0, 1.0, 0.0, 1.0]).into(),
                // bottom left
                ([-0.5, -0.5, 0.5], [1.0, 1.0, 0.0, 1.0]).into(),
                // bottom right
                ([0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 1.0]).into(),
            ];

            let desc = rg::vbuf_desc(verts, rg::ResourceUsage::Immutable, "quad-vertices");
            Buffer::create(&desc)
        };

        // index for 2 triangles
        self.bind.index_buffer = {
            let indices: &[u8] = &[
                0, 1, 1, 2, 2, 0, // first triangle
                3, 2, 2, 1, 1, 3, // second triangle
            ];
            let desc = &rg::ibuf_desc(indices, rg::ResourceUsage::Immutable, "quad-indices");
            Buffer::create(&desc)
        };

        self.pip = {
            let pip_desc = rg::PipelineDesc {
                shader: shaders::quad(),
                index_type: rg::IndexType::UInt16 as u32,
                primitive_type: rg::PrimitiveType::Lines as u32,
                layout: rg::LayoutDesc {
                    attrs: {
                        let mut attrs = [rg::VertexAttrDesc::default(); 16];
                        attrs[0].format = rg::VertexFormat::Float3 as u32;
                        attrs[1].format = rg::VertexFormat::Float4 as u32;
                        attrs
                    },
                    buffers: [rg::BufferLayoutDesc::default();
                        rokol_ffi::gfx::SG_MAX_SHADERSTAGE_BUFFERS as usize],
                },
                ..Default::default()
            };

            Pipeline::create(&pip_desc)
        }
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(self.pip);
        rg::apply_bindings(&self.bind);
        rg::draw(0, 12, 1); // base_elem, n_elems, n_instances
        rg::end_pass();
        rg::commit();
    }
}
