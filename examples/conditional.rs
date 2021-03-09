//! Conditional compilation by renderer
//!
//! See `rokol/build.rs` for more information.

fn main() {
    #[cfg(rokol_gfx = "metal")]
    println!("Using Metal!");

    #[cfg(rokol_gfx = "d3d11")]
    println!("Using D3D11!");

    #[cfg(rokol_gfx = "glcore33")]
    println!("Using GlCORE33!");
}
