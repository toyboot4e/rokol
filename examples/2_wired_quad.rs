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
        rg::setup(&mut rokol::glue::app_desc());

        self.bind.vertex_buffers[0] = Buffer::create({
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

            &rg::vbuf_desc(verts, rg::ResourceUsage::Immutable, "quad-vertices")
        });

        // index for 2 triangles
        self.bind.index_buffer = Buffer::create({
            let indices: &[u8] = &[
                0, 1, 1, 2, 2, 0, // first triangle
                3, 2, 2, 1, 1, 3, // second triangle
            ];
            &rg::ibuf_desc(indices, rg::ResourceUsage::Immutable, "quad-indices")
        });

        self.pip = Pipeline::create({
            &rg::PipelineDesc {
                shader: shaders::quad(),
                index_type: rg::IndexType::UInt16 as u32,
                primitive_type: rg::PrimitiveType::Lines as u32,
                layout: {
                    let mut desc = rg::LayoutDesc::default();
                    desc.attrs[0].format = rg::VertexFormat::Float3 as u32;
                    desc.attrs[1].format = rg::VertexFormat::Float4 as u32;
                    desc
                },
                ..Default::default()
            }
        });
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(self.pip);
        rg::apply_bindings(&self.bind);
        // draw 12 lines (two triangles)
        rg::draw(0, 12, 1); // base_elem, n_indices, n_instances
        rg::end_pass();
        rg::commit();
    }
}
