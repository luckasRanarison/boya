use crate::ppu::{
    Ppu,
    registers::{dispcnt::Background, window::Window},
};

pub const WINDOWS: [Window; 2] = [Window::Win0, Window::Win1];

#[derive(Debug, Default)]
pub struct RenderFlags {
    pub obj: bool,
    pub bg: bool,
    pub effects: bool,
}

impl Ppu {
    pub fn get_render_flags(&self, window: Option<Window>, bg: Background) -> RenderFlags {
        let (obj, bg, effects) = match window {
            Some(Window::Obj) => (
                self.registers.winout.obj_win_obj_enable(),
                self.registers.winout.obj_win_bg_enable(bg),
                self.registers.winout.obj_win_color_fx_enable(),
            ),
            Some(window) => (
                self.registers.winin.obj_enable(window),
                self.registers.winin.bg_enable(window, bg),
                self.registers.winin.color_fx_enable(window),
            ),
            _ if self.has_active_win() => (
                self.registers.winout.obj_enable(),
                self.registers.winout.bg_enable(bg),
                self.registers.winout.color_fx_enable(),
            ),
            _ => (
                self.registers.dispcnt.obj_enable(),
                self.registers.dispcnt.bg_enable(bg),
                true,
            ),
        };

        RenderFlags { obj, bg, effects }
    }

    pub fn has_active_win(&self) -> bool {
        WINDOWS
            .into_iter()
            .any(|win| self.registers.dispcnt.win_enable(win))
    }

    pub fn get_current_win(&self, x: u16, y: u16) -> Option<Window> {
        WINDOWS
            .into_iter()
            .find(|win| self.is_inside_win(*win, x, y))
    }

    fn is_inside_win(&self, win: Window, x: u16, y: u16) -> bool {
        if !self.registers.dispcnt.win_enable(win) {
            return false;
        }

        let winh = &self.registers.winh[win as usize];
        let winv = &self.registers.winv[win as usize];

        x >= winh.x1 as u16 && x < winh.x2 as u16 && y >= winv.y1 as u16 && y < winv.y2 as u16
    }
}
