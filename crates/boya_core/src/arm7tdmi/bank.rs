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
        match op_mode {
            OperatingMode::FIQ => self.psr[0],
            OperatingMode::SVC => self.psr[1],
            OperatingMode::ABT => self.psr[2],
            OperatingMode::IRQ => self.psr[3],
            OperatingMode::UND => self.psr[4],
            _ => unreachable!("tried to read SPSR from {op_mode:?}"),
        }
    }

    pub fn set_spsr(&mut self, op_mode: OperatingMode, cpsr: Psr) {
        match op_mode {
            OperatingMode::FIQ => self.psr[0] = cpsr,
            OperatingMode::SVC => self.psr[1] = cpsr,
            OperatingMode::ABT => self.psr[2] = cpsr,
            OperatingMode::IRQ => self.psr[3] = cpsr,
            OperatingMode::UND => self.psr[4] = cpsr,
            _ => {}
        }
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
}
