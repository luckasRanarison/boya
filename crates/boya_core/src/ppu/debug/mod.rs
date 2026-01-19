use crate::{
    bus::Bus,
    ppu::{
        PALETTE_SIZE, Ppu,
        background::TILE_BUFFER_SIZE,
        color::{Color15, Color24},
        registers::bgcnt::ColorMode,
    },
    utils::bitflags::Bitflag,
};

impl Ppu {
    pub fn color_palette(&self) -> Vec<Color15> {
        self.palette
            .chunks_exact(2)
            .map(|color| u16::from_le_bytes([color[0], color[1]]).into())
            .collect()
    }

    pub fn render_tile(
        &self,
        tile: &[u8],
        color_mode: ColorMode,
        palette_id: usize,
    ) -> Box<[u8; TILE_BUFFER_SIZE]> {
        let mut buffer = [0; TILE_BUFFER_SIZE];
        let palette_start = palette_id * PALETTE_SIZE;

        for i in 0..8 * 8 {
            let color_addr = match color_mode {
                ColorMode::Palette16 => {
                    let pixels = tile[i / 2];
                    let (b_start, b_end) = if i % 2 == 0 { (0, 3) } else { (4, 7) };
                    let color_id = pixels.get_bits_u8(b_start, b_end);
                    palette_start as u32 + color_id as u32 * 2
                }
                ColorMode::Palette256 => {
                    let color_id = tile[i];
                    color_id as u32 * 2
                }
            };

            let raw_color = self.palette.read_hword(color_addr);
            let color15 = Color15::from(raw_color);
            let color24 = Color24::from(color15);

            buffer[i * 4] = color24.r;
            buffer[i * 4 + 1] = color24.g;
            buffer[i * 4 + 2] = color24.b;
            buffer[i * 4 + 3] = 0xFF;
        }

        buffer.into()
    }
}
