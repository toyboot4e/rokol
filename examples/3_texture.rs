//! Draw a texture!

mod shaders;

use {
    image::{io::Reader as ImageReader, GenericImageView},
    rokol::{
        app as ra,
        gfx::{self as rg, BakedResource, Buffer, Pipeline},
    },
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

    rg::Image::create(&{
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

        desc
    })
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
        // now we can call sokol_gfx functions!

        self.bind.fs_images[0] = {
            let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
            let path = root.join("examples/images/RPG Nature Tileset.png");
            self::load_img(&path)
        };

        self.bind.vertex_buffers[0] = Buffer::create({
            let verts: &[Vertex] = &[
                ([-0.5, -0.5, 0.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
                ([0.5, -0.5, 0.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
                ([0.5, 0.5, 0.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
                ([-0.5, 0.5, 0.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            ];

            &rg::vbuf_desc(verts, rg::ResourceUsage::Immutable, "texture-vertices")
        });

        // index for with 2 triangles
        self.bind.index_buffer = Buffer::create({
            let indices: &[u16] = &[0, 1, 2, 0, 2, 3];
            &rg::ibuf_desc(indices, rg::ResourceUsage::Immutable, "texture-indices")
        });

        self.pip = Pipeline::create(&rg::PipelineDesc {
            shader: shaders::texture(),
            index_type: rg::IndexType::UInt16 as u32,
            layout: {
                let mut desc = rg::LayoutDesc::default();
                desc.attrs[0].format = rg::VertexFormat::Float3 as u32;
                desc.attrs[1].format = rg::VertexFormat::UByte4N as u32;
                desc.attrs[2].format = rg::VertexFormat::Float2 as u32;
                desc
            },
            ..Default::default()
        });
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::apply_pipeline(self.pip);
        rg::apply_bindings(&self.bind);
        rg::draw(0, 6, 1); // base_elem, n_indices, n_instances
        rg::end_pass();
        rg::commit();
    }
}
