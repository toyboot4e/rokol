/*!

Rokol fontstash renderer

*/

pub use fontstash::{self, Align, FontStash};

use {
    fontstash::FonsTextIter,
    std::os::raw::{c_int, c_uchar, c_void},
};

use crate::gfx::{self as rg, BakedResource};

#[derive(Debug, Clone)]
pub struct FontConfig {
    pub font: fontstash::FontIx,
    pub fontsize: f32,
    pub line_spacing: f32,
}

/// The shared ownership of [`FontBookImpl`]
///
/// It is required to use [`Box`] to give fixed memory location.
#[derive(Debug)]
pub struct FontBook {
    /// Give fixed memory location
    inner: Box<FontBookImpl>,
}

impl std::ops::Deref for FontBook {
    type Target = FontBookImpl;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for FontBook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FontBook {
    pub fn new(w: u32, h: u32) -> Self {
        let mut inner = Box::new(FontBookImpl {
            stash: FontStash::uninitialized(),
            img: Default::default(),
            w,
            h,
            is_dirty: false,
            tex_data: Vec::with_capacity((w * h) as usize),
        });

        let inner_ptr = inner.as_ref() as *const _ as *mut FontBookImpl;
        // create internal image with the `create` callback:
        inner.stash.init_mut(w, h, inner_ptr);

        fontstash::set_error_callback(
            inner.stash().raw(),
            fons_error_callback,
            inner_ptr as *mut _,
        );

        return FontBook { inner };

        unsafe extern "C" fn fons_error_callback(
            _uptr: *mut c_void,
            error_code: c_int,
            _val: c_int,
        ) {
            match fontstash::ErrorCode::from_u32(error_code as u32) {
                Some(error) => {
                    log::warn!("fons error: {:?}", error);
                }
                None => {
                    log::warn!("fons error error: given broken erroor code");
                }
            }
        }
    }
}

/// The implementation of [`FontBook`]
///
/// It is required to use the internal variable so that the memory position is fixed.
#[derive(Debug)]
pub struct FontBookImpl {
    stash: fontstash::FontStash,
    img: rg::Image,
    /// The texture size is always synced with the fontstash size
    w: u32,
    /// The texture size is always synced with the fontstash size
    h: u32,
    /// We store texture data here because be can update our texture only once a frame
    tex_data: Vec<u8>,
    /// Shall we update the texture data?
    is_dirty: bool,
}

impl Drop for FontBookImpl {
    fn drop(&mut self) {
        log::trace!("fontbook: drop");

        if !self.img.id != 0 {
            rg::Image::destroy(self.img);
        }
    }
}

/// Interface
impl FontBookImpl {
    pub fn img(&self) -> rg::Image {
        self.img
    }

    /// Copies the shared ownership of Fontstash
    pub fn stash(&self) -> FontStash {
        self.stash.clone()
    }

    /// TODO: does it affect performance (dramatically)?
    pub fn apply_cfg(&mut self, cfg: &FontConfig) {
        self.stash.set_font(cfg.font);
        self.stash.set_size(cfg.fontsize);
    }

    /// Be sure to set alignment of the [`FontStash`] to draw text as you want.
    pub fn text_iter(&mut self, text: &str) -> fontstash::Result<FonsTextIter> {
        self.stash.text_iter(text)
    }

    /// [x, y, w, h]
    ///
    /// Be sure to set alignment of the [`FontStash`] to draw text as you want.
    pub fn text_bounds(&self, pos: impl Into<[f32; 2]>, cfg: &FontConfig, text: &str) -> [f32; 4] {
        // TODO: apply `FontConfig` automatially?
        // self.apply_cfg(cfg);
        let mut lines = text.lines();

        let [x, y, mut w, mut h] = {
            let [x1, y1, x2, y2] = self.stash.bounds(pos.into(), lines.next().unwrap());
            [x1, y1, x2 - x1, y2 - y1]
        };

        for line in lines {
            if line.is_empty() {
                h += cfg.fontsize + cfg.line_spacing;
                continue;
            } else {
                let [x1, y1, x2, y2] = self.stash.bounds([0.0, 0.0], line);

                if x2 - x1 > w {
                    w = x2 - x1;
                }

                h += (y2 - y1) + cfg.line_spacing;
            }
        }

        [x, y, w, h]
    }
}

// --------------------------------------------------------------------------------
// Callback and texture updating

/// Renderer implementation
///
/// Return `1` to represent success.
unsafe impl fontstash::Renderer for FontBookImpl {
    /// Creates font texture
    unsafe extern "C" fn create(uptr: *mut c_void, width: c_int, height: c_int) -> c_int {
        let me = &mut *(uptr as *const _ as *mut Self);

        if me.img.id != 0 {
            log::trace!("fontbook: create -- dispose old image");
            rg::Image::destroy(me.img);
        }

        log::trace!("fontbook: create [{}, {}]", width, height);

        me.img = rg::Image::create(&rg::ImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            width,
            height,
            usage: rg::ResourceUsage::Dynamic as u32,
            ..Default::default()
        });

        me.w = width as u32;
        me.h = height as u32;

        me.is_dirty = true;

        true as c_int // success
    }

    unsafe extern "C" fn resize(uptr: *mut c_void, width: c_int, height: c_int) -> c_int {
        log::trace!("fontbook: resize");

        Self::create(uptr, width, height);
        true as c_int // success
    }

    /// Try to double the texture size while the atlas is full
    unsafe extern "C" fn expand(uptr: *mut c_void) -> c_int {
        log::trace!("fontbook: expand");

        let me = &mut *(uptr as *const _ as *mut Self);

        // Self::create(uptr, (me.w * 2) as i32, (me.h * 2) as i32);

        if let Err(why) = me.stash.expand_atlas(me.w * 2, me.h * 2) {
            log::warn!("fontstash: error on resize: {:?}", why);
            false as c_int // fail
        } else {
            true as c_int // success
        }
    }

    unsafe extern "C" fn update(
        uptr: *mut c_void,
        // TODO: what is the dirty rect
        _rect: *mut c_int,
        _data: *const c_uchar,
    ) -> c_int {
        let me = &mut *(uptr as *const _ as *mut Self);
        me.is_dirty = true;
        true as c_int // success
    }
}

impl FontBookImpl {
    /// Updates GPU texure. Call it whenever drawing text
    ///
    /// TODO: do not update twice a frame?
    fn update_cpu_image(&mut self) {
        let tex_data = &mut self.tex_data;
        tex_data.clear();

        self.stash.with_pixels(|pixels, w, h| {
            log::trace!("fontbook: [{}, {}] update CPU texture", w, h);

            let area = (w * h) as usize;
            // self.tex_data.ensure_capacity(area);

            // four channels (RGBA)
            for i in 0..area {
                tex_data.push(255);
                tex_data.push(255);
                tex_data.push(255);
                tex_data.push(pixels[i]);
            }
        });

        // self.w = w;
        // self.h = h;
    }

    /// Call it every frame but only once
    ///
    /// TODO: we may not be ablt to show text on first frame
    pub unsafe fn maybe_update_image(&mut self) {
        if !self.is_dirty {
            return;
        }
        self.is_dirty = false;

        log::trace!("fontbook: [{}, {}] >>> update GPU texture", self.w, self.h);

        self.update_cpu_image();
        rg::update_image(self.img, &{
            let mut content = rg::ImageContent::default();
            content.subimage[0][0] = rg::SubImageContent {
                ptr: self.tex_data.as_ptr() as *mut _,
                size: self.tex_data.len() as i32,
            };
            content
        });
    }
}
