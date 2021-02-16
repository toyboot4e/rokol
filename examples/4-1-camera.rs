/*!
Textured cube with multiple instances

In OpenGL, we use school math (column-major matrices and right-handed coordinate system)
*/

mod shaders;

use {
    glam::{Mat4, Vec2, Vec3},
    image::{io::Reader as ImageReader, GenericImageView},
    rokol::{
        app as ra,
        gfx::{self as rg, BakedResource, Buffer, Pipeline},
    },
    std::{
        f32::consts::PI,
        path::{Path, PathBuf},
    },
};

fn main() -> rokol::Result {
    env_logger::init(); // give implementation to log crate

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Camera".to_string(),
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
    #[cfg(rokol_gfx = "glcore33")]
    let img = img.flipv();

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

        desc.data.subimage[0][0] = pixels.into();
        desc
    })
}

/// From degree to radians
#[inline]
fn rad(degree: f32) -> f32 {
    degree / (2.0 * PI)
}

/// Eular angle
#[derive(Debug, Default, PartialEq)]
pub struct Euler {
    /// Rotation around x axis
    pub yaw: f32,
    /// Rotation around y axis
    pub pitch: f32,
    // /// Rotation around z axis
    // pub roll: f32,
}

impl Euler {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.yaw.cos() * self.pitch.cos(),
            y: self.pitch.sin(),
            z: self.yaw.sin() * self.pitch.cos(),
        }
    }

    pub fn add_pitch(&mut self, pitch: f32) {
        let min = rad(1.0);
        let max = rad(89.0);
        self.pitch = match self.pitch + pitch {
            x if x < min => min,
            x if x > max => max,
            x => x,
        };
    }
}

#[derive(Debug, Default)]
pub struct FlyCamera {
    pub pos: glam::Vec3,
    pub up: glam::Vec3,
    pub fov: f32,
    pub euler: Euler,
    pub front: glam::Vec3,
}

impl FlyCamera {
    pub fn add_fov(&mut self, fov: f32) {
        // FIXME: we can zoom out too much
        let min = rad(1.0);
        let max = rad(89.0);

        self.fov = match self.fov + fov {
            x if x < min => min,
            x if x > max => max,
            x => x,
        };

        log::trace!("FoV = {:?} rad {:?} deg", self.fov, self.fov * 2.0 * PI);
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
    /// Positions of cubes
    cubes: [Vec3; 10],
    cam: FlyCamera,
    last_mouse_pos: Vec2,
}

impl AppData {
    pub fn new() -> Self {
        let color = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];

        let cubes = [
            [0.0, 0.0, 0.0].into(),
            [2.0, 5.0, -15.0].into(),
            [-1.5, -2.2, -2.5].into(),
            [-3.8, -2.0, -12.3].into(),
            [2.4, -0.4, -3.5].into(),
            [-1.7, 3.0, -7.5].into(),
            [1.3, -2.0, -2.5].into(),
            [1.5, 2.0, -2.5].into(),
            [1.5, 0.2, -1.5].into(),
            [-1.3, 1.0, -1.5].into(),
        ];

        Self {
            pa: rg::PassAction::clear(color),
            cubes,
            cam: FlyCamera {
                pos: [0.0, 0.0, 5.0].into(),
                front: [0.0, 0.0, -1.0].into(),
                up: [0.0, 1.0, 0.0].into(),
                fov: PI / 4.0,
                euler: Euler {
                    yaw: -rad(1.0),
                    pitch: 0.0,
                },
            },
            ..Default::default()
        }
    }
}

impl rokol::app::RApp for AppData {
    fn init(&mut self) {
        rg::setup(&mut rokol::glue::app_desc());

        let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        self.bind.fs_images[0] = self::load_img(&root.join("examples/images/container2.png"));
        self.bind.fs_images[1] = self::load_img(&root.join("examples/images/awesomeface.png"));

        self.bind.vertex_buffers[0] = Buffer::create({
            let white = [255 as u8, 255, 255, 255];

            // cube vertices
            let verts: &[Vertex] = &[
                // six rectangles
                ([-0.5, -0.5, -0.5], white, [0.0, 0.0]).into(),
                ([0.5, -0.5, -0.5], white, [1.0, 0.0]).into(),
                ([0.5, 0.5, -0.5], white, [1.0, 1.0]).into(),
                ([-0.5, 0.5, -0.5], white, [0.0, 1.0]).into(),
                //
                ([-0.5, -0.5, 0.5], white, [0.0, 0.0]).into(),
                ([0.5, -0.5, 0.5], white, [1.0, 0.0]).into(),
                ([0.5, 0.5, 0.5], white, [1.0, 1.0]).into(),
                ([-0.5, 0.5, 0.5], white, [0.0, 1.0]).into(),
                //
                ([-0.5, -0.5, -0.5], white, [0.0, 0.0]).into(),
                ([-0.5, 0.5, -0.5], white, [1.0, 0.0]).into(),
                ([-0.5, 0.5, 0.5], white, [1.0, 1.0]).into(),
                ([-0.5, -0.5, 0.5], white, [0.0, 1.0]).into(),
                //
                ([0.5, -0.5, -0.5], white, [0.0, 0.0]).into(),
                ([0.5, 0.5, -0.5], white, [1.0, 0.0]).into(),
                ([0.5, 0.5, 0.5], white, [1.0, 1.0]).into(),
                ([0.5, -0.5, 0.5], white, [0.0, 1.0]).into(),
                //
                ([-0.5, -0.5, -0.5], white, [0.0, 0.0]).into(),
                ([-0.5, -0.5, 0.5], white, [1.0, 0.0]).into(),
                ([0.5, -0.5, 0.5], white, [1.0, 1.0]).into(),
                ([0.5, -0.5, -0.5], white, [0.0, 1.0]).into(),
                //
                ([-0.5, 0.5, -0.5], white, [0.0, 0.0]).into(),
                ([-0.5, 0.5, 0.5], white, [1.0, 0.0]).into(),
                ([0.5, 0.5, 0.5], white, [1.0, 1.0]).into(),
                ([0.5, 0.5, -0.5], white, [0.0, 1.0]).into(),
            ];

            &rg::vbuf_desc_immutable(verts, "texcube-vertices")
        });

        self.bind.index_buffer = Buffer::create({
            let indices: &[u16] = &[
                0, 1, 2, 0, 2, 3, // one rectangle
                6, 5, 4, 7, 6, 4, //
                8, 9, 10, 8, 10, 11, //
                14, 13, 12, 15, 14, 12, //
                16, 17, 18, 16, 18, 19, //
                22, 21, 20, 23, 22, 20, //
            ];
            &rg::ibuf_desc_immutable(indices, "texcube-indices")
        });

        self.pip = Pipeline::create(&rg::PipelineDesc {
            layout: {
                let mut desc = rg::LayoutDesc::default();
                desc.attrs[0].format = rg::VertexFormat::Float3 as u32;
                desc.attrs[1].format = rg::VertexFormat::UByte4N as u32;
                desc.attrs[2].format = rg::VertexFormat::Float2 as u32;
                desc
            },
            shader: shaders::more_cubes(),
            index_type: rg::IndexType::UInt16 as u32,
            depth: rg::DepthState {
                compare: rg::CompareFunc::LessEq as u32,
                write_enabled: true,
                ..Default::default()
            },
            cull_mode: rg::CullMode::Back as u32,
            ..Default::default()
        });
    }

    fn event(&mut self, ev: &ra::Event) {
        let move_speed = 1.0;
        let zx_zoom_sensitivity = 1.0;
        let mouse_zoom_sensitivity = 0.25;
        let rot_sensitivity = 0.001;

        if ev.type_ == ra::EventType::KeyDown as u32 {
            let key = ra::Key::from_u32(ev.key_code).unwrap();

            // move
            self.cam.pos += match key {
                ra::Key::W => self.cam.front,
                ra::Key::S => -self.cam.front,
                ra::Key::A => -self.cam.front.cross(self.cam.up),
                ra::Key::D => self.cam.front.cross(self.cam.up),
                _ => Vec3::new(0.0, 0.0, 0.0),
            } * move_speed;

            // zoom
            self.cam.add_fov(
                match key {
                    // zoom in (increase FoV)
                    ra::Key::Z => -rad(1.0),
                    // zoom out (decrease FoV)
                    ra::Key::X => rad(1.0),
                    _ => 0.0,
                } * zx_zoom_sensitivity,
            );
        }

        // zoom
        if ev.type_ == ra::EventType::MouseScroll as u32 {
            self.cam.add_fov(ev.scroll_y * mouse_zoom_sensitivity);
        }

        // rotate
        if ev.type_ == ra::EventType::MouseMove as u32 {
            let delta = Vec2::new(ev.mouse_dx, ev.mouse_dy) * rot_sensitivity;

            self.cam.euler.yaw += delta.x;
            self.cam.euler.add_pitch(-delta.y);
            self.cam.front = self.cam.euler.to_vec3();
        }
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        {
            rg::apply_pipeline(self.pip);
            rg::apply_bindings(&self.bind);

            let view = Mat4::look_at_rh(
                // camera position
                self.cam.pos,
                // focal point
                self.cam.pos + self.cam.front,
                // up direction
                self.cam.up,
            );

            let ratio = ra::width() as f32 / ra::height() as f32;
            let proj = Mat4::perspective_rh(
                self.cam.fov, // fov
                ratio,        // aspect_ratio
                0.01,         // z_near
                100.0,        // z_far
            );

            let vp = proj * view;

            // for each cube positions
            for (i, pos) in self.cubes.iter().enumerate() {
                let m = {
                    let m = Mat4::from_translation(pos.clone());
                    let angle = PI / 9.0 * i as f32;
                    m * Mat4::from_axis_angle([1.0, 0.3, 0.5].into(), angle)
                };
                let mvp = vp * m;
                let bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(
                        mvp.as_ref() as *const _ as *const _,
                        std::mem::size_of::<Mat4>(),
                    )
                };
                rg::apply_uniforms(rg::ShaderStage::Vs, 0, bytes);

                rg::draw(0, 36, 1); // base_elem, n_indices, n_instances
            }
        }
        rg::end_pass();
        rg::commit();
    }
}
