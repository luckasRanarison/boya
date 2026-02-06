use crate::{
    bus::Bus,
    ppu::{
        Ppu, TransformParam,
        character::{CharacterData, CharacterKind},
        color::Color15,
        registers::{bgcnt::ColorMode, dispcnt::Background},
    },
    utils::bitflags::Bitflag,
};

#[derive(Debug)]
pub struct Obj {
    attr: [u16; 3],
}

impl Obj {
    pub fn y(&self) -> u16 {
        self.attr[0].get_bits(0, 7)
    }

    pub fn transform(&self) -> bool {
        self.attr[0].has(8)
    }

    pub fn double_size(&self) -> bool {
        self.attr[0].has(9)
    }

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
                self.oam.read_hword(id as u32 * 4),
                self.oam.read_hword(id as u32 * 4 + 1),
                self.oam.read_hword(id as u32 * 4 + 2),
            ],
        }
    }

    pub fn get_obj_transform_params(&self, obj: &Obj) -> TransformParam {
        let id = obj.transform_parameter() as u32;

        TransformParam {
            pa: self.oam.read_hword(id * 16 + 3),
            pb: self.oam.read_hword(id * 16 + 7),
            pc: self.oam.read_hword(id * 16 + 11),
            pd: self.oam.read_hword(id * 16 + 15),
            x: 0,
            y: 0,
        }
    }

    pub fn read_obj_palette(&self, index: u32) -> Color15 {
        self.palette.read_hword(512 + index * 2).into()
    }

    pub fn load_obj_pool(&mut self) {
        if !self.registers.dispcnt.is_obj_enabled() {
            return;
        }

        for id in 0..128 {
            let obj = self.get_object(id);
            let (_width, height) = obj.dimmensions();
            let top = obj.y();
            let bottom = top + height as u16;
            let scanline = self.scanline as u16;

            if scanline >= top && scanline < bottom {
                self.pipeline.obj_pool.push(obj);
            }
        }
    }

    pub fn get_obj_pixel(&self, x: u16, y: u16, layer: Background) -> Option<Color15> {
        if !self.registers.dispcnt.is_obj_enabled() {
            return None;
        }

        let obj = self.pipeline.obj_pool.get(x, layer)?;
        let cx = x - obj.x();
        let cy = y - obj.y();
        let (width, height) = obj.dimmensions();
        let transform = obj.transform().then(|| self.get_obj_transform_params(obj));
        let vram_mapping = self.registers.dispcnt.obj_vram_mapping();

        let char_data = CharacterData {
            name: obj.character().into(),
            base_offset: 0x10_000,
            hflip: obj.hflip(),
            vflip: obj.vflip(),
            color: obj.color_mode(),
            palette: obj.palette(),
            kind: CharacterKind::Object(vram_mapping),
            height,
            width,
            transform,
        };

        self.get_char_pixel(cx, cy, char_data)
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
    pub fn push(&mut self, value: Obj) {
        self.pool[self.len] = value;
        self.len += 1;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn get(&self, x: u16, layer: Background) -> Option<&Obj> {
        for obj in &self.pool[..self.len] {
            if obj.bg_priority() == layer as u8 {
                let (width, _height) = obj.dimmensions();
                let left = obj.x();
                let right = left + width as u16;

                if x >= left && x < right {
                    return Some(obj);
                }
            }
        }

        None
    }
}
