use crate::arm7tdmi::common::OperatingMode;

use super::psr::Psr;

#[derive(Debug, Default)]
pub struct Bank {
    fiq: [u32; 7], // R08-R14
    svc: [u32; 2], // R13-R14
    abt: [u32; 2], // R13-R14
    irq: [u32; 2], // R13-R14
    und: [u32; 2], // R13-R14
    psr: [Psr; 5],
}

impl Bank {
    pub fn get_reg(&self, op_mode: OperatingMode, index: usize) -> Option<u32> {
        self.get_bank(op_mode)
            .filter(|(_, offset)| index >= *offset && index <= 14)
            .map(|(slice, offset)| slice[index - offset])
    }

    pub fn get_reg_mut(&mut self, op_mode: OperatingMode, index: usize) -> Option<&mut u32> {
        self.get_bank_mut(op_mode)
            .filter(|(_, offset)| index >= *offset && index <= 14)
            .map(|(slice, offset)| &mut slice[index - offset])
    }

    pub fn get_spsr(&self, op_mode: OperatingMode) -> Psr {
        self.psr[self.operating_mode_index(op_mode)]
    }

    pub fn set_spsr(&mut self, op_mode: OperatingMode, psr: Psr) {
        self.psr[self.operating_mode_index(op_mode)] = psr;
    }

    pub fn update_spsr(&mut self, op_mode: OperatingMode, fields: u32, mask: u32) {
        let index = self.operating_mode_index(op_mode);
        let value = (self.psr[index].value() & !mask) | fields;

        self.psr[index] = Psr::from(value);
    }

    fn get_bank(&self, op_mode: OperatingMode) -> Option<(&[u32], usize)> {
        match op_mode {
            OperatingMode::FIQ => Some((&self.fiq, 8)),
            OperatingMode::SVC => Some((&self.svc, 13)),
            OperatingMode::ABT => Some((&self.abt, 13)),
            OperatingMode::IRQ => Some((&self.irq, 13)),
            OperatingMode::UND => Some((&self.und, 13)),
            _ => None,
        }
    }

    fn get_bank_mut(&mut self, op_mode: OperatingMode) -> Option<(&mut [u32], usize)> {
        match op_mode {
            OperatingMode::FIQ => Some((&mut self.fiq, 8)),
            OperatingMode::SVC => Some((&mut self.svc, 13)),
            OperatingMode::ABT => Some((&mut self.abt, 13)),
            OperatingMode::IRQ => Some((&mut self.irq, 13)),
            OperatingMode::UND => Some((&mut self.und, 13)),
            _ => None,
        }
    }

    fn operating_mode_index(&self, op_mode: OperatingMode) -> usize {
        match op_mode {
            OperatingMode::FIQ => 0,
            OperatingMode::SVC => 1,
            OperatingMode::ABT => 2,
            OperatingMode::IRQ => 3,
            OperatingMode::UND => 4,
            _ => unreachable!("trying to access PSR for invalid operating mode: {op_mode:?}"),
        }
    }
}
