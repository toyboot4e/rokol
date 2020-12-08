//! Draw a triangle!

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
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
    /// Vertex layouts, shader and render states
    pip: rg::Pipeline,
    /// Vertex/index buffer and image slots
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
                // (vertex, color)
                ([0.0, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]).into(), // top
                ([0.5, -0.5, 0.5], [0.0, 1.0, 0.0, 1.0]).into(), // bottom right
                ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 1.0]).into(), // bottom left
            ];

            &rg::vbuf_desc(verts, rg::ResourceUsage::Immutable, "triangle-vertices")
        });

        self.pip = Pipeline::create(&rg::PipelineDesc {
            shader: shaders::triangle(),
            layout: {
                let mut desc = rg::LayoutDesc::default();
                desc.attrs[0].format = rg::VertexFormat::Float3 as u32;
                desc.attrs[1].format = rg::VertexFormat::Float4 as u32;
                desc
            },
            ..Default::default()
        });
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(self.pip);
        rg::apply_bindings(&self.bind);
        rg::draw(0, 3, 1); // base_elem, n_indices, n_instances
        rg::end_pass();
        rg::commit();
    }
}
