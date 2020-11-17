//! Draw a texture!

mod shaders;

use rokol::{app as ra, gfx as rg};

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

fn main() -> rokol::Result {
    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Window".to_string(),
        ..Default::default()
    };

    let mut app = AppData::new();

    rokol.run(&mut app)
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
        let mut desc = rokol::create_app_desc();
        rg::setup(&mut desc); // now we can call sokol_gfx functions!

        self.bind.vertex_buffers[0] = {
            let buf: &[Vertex] = &[
                // vertex, color
                ([0.0, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]).into(),
                ([0.5, -0.5, 0.5], [0.0, 1.0, 0.0, 1.0]).into(),
                ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 1.0]).into(),
            ];

            let buf_desc =
                rg::BufferDesc::new(buf, rg::ResourceUsage::Immutable, "triangle-vertices");
            rg::make_buffer(&buf_desc)
        };

        let pip_desc = rg::PipelineDesc {
            shader: shaders::make_quad_shader(),
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

        self.pip = rg::make_pipeline(&pip_desc);
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(&self.pip);
        rg::apply_bindings(&self.bind);
        rg::draw(0, 3, 1);
        rg::end_pass();
        rg::commit();
    }
}
