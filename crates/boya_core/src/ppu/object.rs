use crate::{
    bus::Bus,
    ppu::{
        Ppu, TransformParam,
        character::{CharacterData, CharacterKind},
        color::Color15,
        registers::{
            bgcnt::ColorMode,
            dispcnt::{Background, BgMode},
        },
    },
    utils::bitflags::Bitflag,
};

#[derive(Debug)]
pub struct Obj {
    attr: [u16; 3],
}

impl Obj {
    pub fn y(&self) -> u8 {
        self.attr[0].get_bits_u8(0, 7)
    }

    pub fn transform(&self) -> bool {
        self.attr[0].has(8)
    }

    pub fn double_size(&self) -> bool {
        self.attr[0].has(9)
    }

    /// # Panics
    ///
    /// Panics if the prohibited code is used.
    pub fn mode(&self) -> ObjMode {
        match self.attr[0].get_bits(10, 11) {
            0 => ObjMode::Normal,
            1 => ObjMode::SemiTransparent,
            2 => ObjMode::Window,
            _ => panic!("invalid obj mode, prohibited code"),
        }
    }

    pub fn mosaic(&self) -> bool {
        self.attr[0].has(12)
    }

    pub fn color_mode(&self) -> ColorMode {
        match self.attr[0].get(13) {
            0 => ColorMode::Palette16,
            _ => ColorMode::Palette256,
        }
    }

    pub fn x(&self) -> u16 {
        self.attr[1].get_bits(0, 8)
    }

    pub fn transform_parameter(&self) -> u8 {
        self.attr[1].get_bits_u8(9, 13)
    }

    pub fn hflip(&self) -> bool {
        self.attr[1].has(12)
    }

    pub fn vflip(&self) -> bool {
        self.attr[1].has(13)
    }

    /// # Panics
    ///
    /// Panics if a prohibited code is used.
    pub fn dimmensions(&self) -> (u8, u8) {
        let shape = self.attr[0].get_bits_u8(14, 15);
        let size = self.attr[1].get_bits_u8(14, 15);

        match (shape, size) {
            (0, 0) => (8, 8),
            (0, 1) => (16, 16),
            (0, 2) => (32, 32),
            (0, 3) => (64, 64),
            (1, 0) => (16, 8),
            (1, 1) => (32, 8),
            (1, 2) => (32, 16),
            (1, 3) => (64, 32),
            (2, 0) => (8, 16),
            (2, 1) => (8, 32),
            (2, 2) => (16, 32),
            (2, 3) => (32, 64),
            _ => panic!("invalid dimmensions, prohibited code"),
        }
    }

    pub fn character(&self) -> u16 {
        self.attr[2].get_bits(0, 9)
    }

    pub fn bg_priority(&self) -> u8 {
        self.attr[2].get_bits_u8(10, 11)
    }

    pub fn palette(&self) -> u8 {
        self.attr[2].get_bits_u8(12, 15)
    }

    const fn placeholder() -> Self {
        Self { attr: [0; 3] }
    }
}

impl Ppu {
    pub fn get_object(&self, id: u8) -> Obj {
        Obj {
            attr: [
                self.oam.read_hword(id as u32 * 8),
                self.oam.read_hword(id as u32 * 8 + 2),
                self.oam.read_hword(id as u32 * 8 + 4),
            ],
        }
    }

    pub fn get_obj_transform_params(&self, obj: &Obj) -> TransformParam {
        // FIXME
        let id = obj.transform_parameter() as u32;

        TransformParam {
            pa: self.oam.read_hword(id * 32 + 6),
            pb: self.oam.read_hword(id * 32 + 14),
            pc: self.oam.read_hword(id * 32 + 22),
            pd: self.oam.read_hword(id * 32 + 30),
            x: 0,
            y: 0,
        }
    }

    pub fn read_obj_palette(&self, id: u8) -> Color15 {
        self.palette.read_hword(512 + id as u32 * 2).into()
    }

    pub fn load_obj_pool(&mut self) {
        if !self.registers.dispcnt.obj_enable() {
            return;
        }

        self.pipeline.obj_pool.clear();

        for id in 0..128 {
            let obj = self.get_object(id);
            let (_width, height) = obj.dimmensions();
            let height = height * if obj.double_size() { 2 } else { 1 };
            let diff = self.scanline.wrapping_sub(obj.y());

            if diff < height {
                self.pipeline.obj_pool.push(obj);
            }
        }
    }

    pub fn get_obj_pixel(&self, x: u16, y: u16, layer: Background) -> Option<Color15> {
        let mut offset = 0;

        loop {
            let (id, obj) = self.pipeline.obj_pool.get(x, layer as u8, offset)?;
            let cx = x.wrapping_sub(obj.x()) & 0x1FF;
            let cy = y.wrapping_sub(obj.y().into()) & 0xFF;

            if let Some(pixel) = self.get_obj_pixel_inner(cx, cy, obj) {
                return Some(pixel);
            } else {
                offset = id + 1
            }
        }
    }

    pub fn get_obj_pixel_inner(&self, x: u16, y: u16, obj: &Obj) -> Option<Color15> {
        let (width, height) = obj.dimmensions();
        let vram_mapping = self.registers.dispcnt.obj_vram_mapping();

        let (hflip, vflip, transform) = match obj.transform() {
            false => (obj.hflip(), obj.vflip(), None),
            true => (false, false, self.get_obj_transform_params(obj).into()),
        };

        let base_offset = match self.registers.dispcnt.bg_mode() {
            BgMode::Mode0 | BgMode::Mode1 | BgMode::Mode2 => 0x10000,
            BgMode::Mode3 | BgMode::Mode4 | BgMode::Mode5 => 0x14000,
        };

        let char_data = CharacterData {
            name: obj.character(),
            color: obj.color_mode(),
            palette: obj.palette(),
            kind: CharacterKind::Object(vram_mapping),
            width,
            height,
            hflip,
            vflip,
            transform,
            base_offset,
        };

        self.get_char_pixel(x, y, &char_data)
    }
}

#[derive(Debug)]
pub enum ObjMode {
    Normal,
    SemiTransparent,
    Window,
}

#[derive(Debug)]
pub struct ObjPool {
    pool: [Obj; 128],
    len: usize,
}

impl Default for ObjPool {
    fn default() -> Self {
        Self {
            pool: [const { Obj::placeholder() }; 128],
            len: 0,
        }
    }
}

impl ObjPool {
    fn push(&mut self, value: Obj) {
        self.pool[self.len] = value;
        self.len += 1;
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn get(&self, x: u16, layer: u8, offset: usize) -> Option<(usize, &Obj)> {
        if offset > self.len {
            return None;
        }

        for (i, obj) in self.pool[offset..self.len].iter().enumerate() {
            let prio = obj.bg_priority();

            if prio == layer {
                let (width, _height) = obj.dimmensions();
                let width = width * if obj.double_size() { 2 } else { 1 };
                let diff = x.wrapping_sub(obj.x()) & 0x1FF;

                if diff < width as u16 {
                    return Some((offset + i, obj));
                }
            }

            if prio > layer {
                break;
            }
        }

        None
    }
}
