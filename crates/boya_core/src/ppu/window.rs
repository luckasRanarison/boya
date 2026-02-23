use crate::ppu::{
    Ppu,
    registers::{dispcnt::Background, window::Window},
};

pub const WINDOWS: [Window; 2] = [Window::Win0, Window::Win1];

impl Ppu {
    pub fn window_bg_enable(&self, window: Option<Window>, bg: Background) -> bool {
        match window {
            Some(Window::Obj) => self.registers.winout.obj_win_bg_enable(bg),
            Some(window) => self.registers.winin.bg_enable(window, bg),
            _ if self.pipeline.window_enabled => self.registers.winout.bg_enable(bg),
            _ => self.registers.dispcnt.bg_enable(bg),
        }
    }

    pub fn window_obj_enable(&self, window: Option<Window>) -> bool {
        match window {
            Some(Window::Obj) => self.registers.winout.obj_win_obj_enable(),
            Some(window) => self.registers.winin.obj_enable(window),
            _ if self.pipeline.window_enabled => self.registers.winout.obj_enable(),
            _ => self.registers.dispcnt.obj_enable(),
        }
    }

    pub fn window_fx_enable(&self, window: Option<Window>) -> bool {
        match window {
            Some(Window::Obj) => self.registers.winout.obj_win_color_fx_enable(),
            Some(window) => self.registers.winin.color_fx_enable(window),
            _ if self.pipeline.window_enabled => self.registers.winout.color_fx_enable(),
            _ => true,
        }
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
