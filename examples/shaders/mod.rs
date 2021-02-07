/*!

Shaders

Shader files are conditionally embedded to the source code.

See `rokol/build.rs` for the conditional compiltion information.
*/

// NOTE: Be sure to set uniform names (or maybe fail).
// TODO: compile while allowing non-existing shader file

use rokol::gfx::{self as rg, BakedResource, Shader};

/// Creates a null-terminated string from static string
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0");
    };
}

fn gen(vs_fs: &[impl AsRef<str>; 2], f: impl Fn(&mut rg::ShaderDesc)) -> rg::Shader {
    let mut desc = unsafe { rokol::gfx::shader_desc(vs_fs[0].as_ref(), vs_fs[1].as_ref()) };
    f(&mut desc);
    Shader::create(&desc)
}

macro_rules! embed_shd {
    ($vs:expr, $fs:expr,) => {
        if cfg!(debug_assertions) {
            // debug: dynamically load the shader files
            let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("examples/shaders");
            let mut v = std::fs::read_to_string(dir.join($vs)).unwrap();
            v.push('\0');
            let mut f = std::fs::read_to_string(dir.join($fs)).unwrap();
            f.push('\0');
            [v, f]
        } else {
            // release: statically load the shader files
            [
                concat!(include_str!($vs), "\0").to_string(),
                concat!(include_str!($fs), "\0").to_string(),
            ]
        }
    };
}

#[cfg(rokol_gfx = "glcore33")]
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("glsl/", $file, ".vs"),
            concat!("glsl/", $file, ".fs"),
        )
    };
}

#[cfg(rokol_gfx = "metal")]
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("metal/", $file, "_vs.metal"),
            concat!("metal/", $file, "_fs.metal"),
        )
    };
}

#[cfg(rokol_gfx = "d3d11")]
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("d3d11/", $file, "_vs.hlsl"),
            concat!("d3d11/", $file, "_fs.hlsl"),
        )
    };
}

pub fn triangle() -> rokol::gfx::Shader {
    gen(&def_shd!("triangle"), |_desc| {})
}

pub fn quad() -> rokol::gfx::Shader {
    gen(&def_shd!("quad"), |_desc| {})
}

pub fn texture() -> rokol::gfx::Shader {
    gen(&def_shd!("texture"), |desc| {
        desc.fs.images[0] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            ..Default::default()
        };
    })
}

pub fn texture_multi() -> rokol::gfx::Shader {
    gen(&def_shd!("texture_multi"), |desc| {
        desc.fs.images[0] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex1").as_ptr() as *const _,
            ..Default::default()
        };
        desc.fs.images[1] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex2").as_ptr() as *const _,
            ..Default::default()
        };
    })
}

pub fn cube() -> rokol::gfx::Shader {
    gen(&def_shd!("cube"), |desc| {
        desc.vs.uniform_blocks[0] = {
            let mut block = rg::ShaderUniformBlockDesc::default();
            block.size = std::mem::size_of::<glam::Mat4>() as i32;
            block.uniforms[0] = rg::ShaderUniformDesc {
                type_: rg::UniformType::Mat4 as u32,
                name: c_str!("mvp").as_ptr() as *const _,
                ..Default::default()
            };
            block
        };

        desc.fs.images[0] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex").as_ptr() as *const _,
            ..Default::default()
        };
    })
}

pub fn cube_multi() -> rokol::gfx::Shader {
    gen(&def_shd!("cube_multi"), |desc| {
        desc.vs.uniform_blocks[0] = {
            let mut block = rg::ShaderUniformBlockDesc::default();
            block.size = std::mem::size_of::<glam::Mat4>() as i32;
            block.uniforms[0] = rg::ShaderUniformDesc {
                type_: rg::UniformType::Mat4 as u32,
                name: c_str!("mvp").as_ptr() as *const _,
                ..Default::default()
            };
            block
        };

        desc.fs.images[0] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex1").as_ptr() as *const _,
            ..Default::default()
        };

        desc.fs.images[1] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex2").as_ptr() as *const _,
            ..Default::default()
        };
    })
}

pub fn more_cubes() -> rokol::gfx::Shader {
    gen(&def_shd!("more_cubes"), |desc| {
        desc.vs.uniform_blocks[0] = {
            let mut block = rg::ShaderUniformBlockDesc::default();
            block.size = std::mem::size_of::<glam::Mat4>() as i32;
            block.uniforms[0] = rg::ShaderUniformDesc {
                type_: rg::UniformType::Mat4 as u32,
                name: c_str!("mvp").as_ptr() as *const _,
                ..Default::default()
            };
            block
        };

        desc.fs.images[0] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex1").as_ptr() as *const _,
            ..Default::default()
        };

        desc.fs.images[1] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            name: c_str!("tex2").as_ptr() as *const _,
            ..Default::default()
        };
    })
}
