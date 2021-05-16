/*!
FontStash integration for Rokol
*/

pub use fontstash::{self, Align, FonsQuad, FontStash};

use std::os::raw::{c_int, c_uchar, c_void};

use crate::gfx::{self as rg, BakedResource};

/// Be sure to set alignment of the [`FontStash`] to draw text as you want.
#[derive(Debug)]
pub struct FontTexture {
    /// Give fixed memory location
    inner: Box<FontTextureImpl>,
}

impl std::ops::Deref for FontTexture {
    type Target = FontTextureImpl;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for FontTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FontTexture {
    pub fn new(w: u32, h: u32) -> Self {
        let mut inner = Box::new(FontTextureImpl {
            stash: FontStash::uninitialized(),
            img: Default::default(),
            w,
            h,
            is_dirty: false,
            tex_data: Vec::with_capacity((w * h) as usize),
        });

        let inner_ptr = inner.as_ref() as *const _ as *mut FontTextureImpl;
        // create internal image with the `create` callback:
        inner.stash.init_mut(w, h, inner_ptr);

        fontstash::set_error_callback(
            inner.stash().raw(),
            fons_error_callback,
            inner_ptr as *mut _,
        );

        return FontTexture { inner };

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

/// We have to give fixed memory location to `FontTextureImpl` so that `fontstash` (a C library) can
/// call callback methods.
#[derive(Debug)]
pub struct FontTextureImpl {
    stash: fontstash::FontStash,
    img: rg::Image,
    /// The texture size, which is always synced with the fontstash size
    w: u32,
    /// The texture size, which is always synced with the fontstash size
    h: u32,
    /// We store texture data here because be can update our texture only once a frame
    tex_data: Vec<u8>,
    /// Shall we update the texture data in this frame?
    is_dirty: bool,
}

impl Drop for FontTextureImpl {
    fn drop(&mut self) {
        log::trace!("FontTextureImpl::drop");

        if self.img.id != 0 {
            log::trace!("==> destroy GPU font texture");
            rg::Image::destroy(self.img);
        }
    }
}

impl std::ops::Deref for FontTextureImpl {
    type Target = FontStash;
    fn deref(&self) -> &Self::Target {
        &self.stash
    }
}

impl std::ops::DerefMut for FontTextureImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stash
    }
}

/// Interface
impl FontTextureImpl {
    pub fn img(&self) -> rg::Image {
        self.img
    }

    pub fn cpu_texture(&self) -> (&Vec<u8>, [u32; 2]) {
        (&self.tex_data, [self.w, self.h])
    }

    pub fn stash(&self) -> &FontStash {
        &self.stash
    }

    /// Copies the shared ownership of Fontstash
    /// Returns [x, y, w, h]
    pub fn text_bounds_multiline(
        &self,
        text: &str,
        pos: impl Into<[f32; 2]>,
        fontsize: f32,
        line_spacing: f32,
    ) -> [f32; 4] {
        // TODO: apply fontsize automatially?
        let mut lines = text.lines();

        self.stash.set_size(fontsize);

        let [x, y, mut w, mut h] = {
            let [x1, y1, x2, y2] = self
                .stash
                .text_bounds_oneline(pos.into(), lines.next().unwrap());
            [x1, y1, x2 - x1, y2 - y1]
        };

        for line in lines {
            if line.is_empty() {
                h += fontsize + line_spacing;
            } else {
                let [x1, y1, x2, y2] = self.stash.text_bounds_oneline([0.0, 0.0], line);

                if x2 - x1 > w {
                    w = x2 - x1;
                }

                // h += (y2 - y1) + line_spacing;
                h += fontsize + line_spacing;
            }
        }

        [x, y, w, h]
    }

    /// Returns [x, y, w, h]
    pub fn text_size_multiline(&self, text: &str, fontsize: f32, line_spacing: f32) -> [f32; 2] {
        // TODO: apply fontsize automatially?
        let mut lines = text.lines();

        self.stash.set_size(fontsize);

        let [mut w, mut h] = self.stash.text_size_oneline(lines.next().unwrap());

        for line in lines {
            if line.is_empty() {
                h += fontsize + line_spacing;
                continue;
            } else {
                let [w2, h2] = self.stash.text_size_oneline(line);

                if w2 > w {
                    w = w2
                }

                // h += h2 + line_spacing;
                h += fontsize + line_spacing;
            }
        }

        [w, h]
    }
}

// --------------------------------------------------------------------------------
// Callback and texture updating

/// Renderer implementation
///
/// Return `1` to represent success.
unsafe impl fontstash::Renderer for FontTextureImpl {
    /// Creates font texture
    unsafe extern "C" fn create(uptr: *mut c_void, width: c_int, height: c_int) -> c_int {
        let me = &mut *(uptr as *const _ as *mut Self);

        if me.img.id != 0 {
            log::trace!("FontTextureImpl::create -- dispose old image");
            rg::Image::destroy(me.img);
        }

        log::trace!("FontTextureImpl::create [{}, {}]", width, height);

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
        log::trace!("FontTextureImpl::resize");

        Self::create(uptr, width, height);
        true as c_int // success
    }

    /// Try to double the texture size when the atlas is full
    unsafe extern "C" fn expand(uptr: *mut c_void) -> c_int {
        log::trace!("FontTextureImpl::expand");

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

impl FontTextureImpl {
    fn update_cpu_image(&mut self) {
        let tex_data = &mut self.tex_data;
        tex_data.clear();

        self.stash.with_pixels(|pixels, w, h| {
            log::trace!("FontTextureImpl: [{}, {}] update CPU texture", w, h);

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
    pub unsafe fn maybe_update_image(&mut self) {
        if !self.is_dirty {
            return;
        }
        self.is_dirty = false;

        self.update_cpu_image();
        rg::update_image(self.img, &{
            let mut data = rg::ImageData::default();
            data.subimage[0][0] = self.tex_data.as_slice().into();
            data
        });
    }
}
