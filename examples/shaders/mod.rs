pub mod glsl {
    pub static SIMPLE_VS: &str = include_str!("glsl/simple_vs.glsl");
    pub static SIMPLE_FS: &str = include_str!("glsl/simple_fs.glsl");
}

pub mod metal {
    pub static SIMPLE_VS: &str = include_str!("metal/simple_vs.metal");
    pub static SIMPLE_FS: &str = include_str!("metal/simple_fs.metal");
}

// pub fn
