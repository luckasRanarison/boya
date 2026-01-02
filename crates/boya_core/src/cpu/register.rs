use crate::cpu::{common::OperatingMode, psr::Psr};

#[derive(Default)]
pub struct Register {
    pub main: [u32; 16], // R0-R15

    fiq: [u32; 7], // R08-R14
    svc: [u32; 2], // R13-R14
    abt: [u32; 2], // R13-R14
    irq: [u32; 2], // R13-R14
    und: [u32; 2], // R13-R14
    psr: [Psr; 5],
}

impl Register {
    pub const PC: usize = 15;
    pub const LR: usize = 14;
    pub const SP: usize = 13;

    pub fn get<I: Into<usize>>(&self, index: I, op_mode: OperatingMode) -> u32 {
        let index = index.into();

        match (op_mode, index) {
            (OperatingMode::FIQ, 8..=14) => self.fiq[index - 8],
            (OperatingMode::SVC, 13..=14) => self.svc[index - 13],
            (OperatingMode::ABT, 13..=14) => self.abt[index - 13],
            (OperatingMode::IRQ, 13..=14) => self.irq[index - 13],
            (OperatingMode::UND, 13..=14) => self.und[index - 13],
            _ => self.main[index],
        }
    }

    pub fn set<I: Into<usize>>(&mut self, index: I, value: u32, op_mode: OperatingMode) {
        let index = index.into();

        match (op_mode, index) {
            (OperatingMode::FIQ, 8..=14) => self.fiq[index - 8] = value,
            (OperatingMode::SVC, 13..=14) => self.svc[index - 13] = value,
            (OperatingMode::ABT, 13..=14) => self.abt[index - 13] = value,
            (OperatingMode::IRQ, 13..=14) => self.irq[index - 13] = value,
            (OperatingMode::UND, 13..=14) => self.und[index - 13] = value,
            _ => self.main[index] = value,
        }
    }

    pub fn get_spsr(&self, op_mode: OperatingMode) -> Option<Psr> {
        self.operating_mode_index(op_mode)
            .map(|index| self.psr[index])
    }

    pub fn get_spsr_unchecked(&self, op_mode: OperatingMode) -> Psr {
        self.get_spsr(op_mode)
            .unwrap_or_else(|| panic!("invalid SPSR access, mode: {op_mode:?}"))
    }

    pub fn set_spsr(&mut self, op_mode: OperatingMode, psr: Psr) {
        if let Some(index) = self.operating_mode_index(op_mode) {
            self.psr[index] = psr;
        }
    }

    pub fn update_spsr(&mut self, op_mode: OperatingMode, fields: u32, mask: u32) {
        if let Some(index) = self.operating_mode_index(op_mode) {
            let psr = self.psr[index];
            let value = (psr.value() & !mask) | fields;

            self.psr[index] = Psr::from(value);
        }
    }

    #[inline(always)]
    pub fn set_pc(&mut self, value: u32) {
        self.main[Self::PC] = value;
    }

    #[inline(always)]
    pub fn pc(&self) -> u32 {
        self.main[Self::PC]
    }

    #[inline(always)]
    pub fn shift_pc(&mut self, offset: i32) {
        self.main[Self::PC] = self.main[Self::PC].wrapping_add_signed(offset);
    }

    fn operating_mode_index(&self, op_mode: OperatingMode) -> Option<usize> {
        match op_mode {
            OperatingMode::FIQ => Some(0),
            OperatingMode::SVC => Some(1),
            OperatingMode::ABT => Some(2),
            OperatingMode::IRQ => Some(3),
            OperatingMode::UND => Some(4),
            _ => None,
        }
    }
}
