use super::psr::{OperatingMode, Psr};

#[derive(Debug, Default)]
pub struct Bank {
    fiq: [u32; 8], // R08-R14 + SPSR
    svc: [u32; 3], // R13-R14 + SPSR
    abt: [u32; 3], // R13-R14 + SPSR
    irq: [u32; 3], // R13-R14 + SPSR
    und: [u32; 3], // R13-R14 + SPSR
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

    pub fn get_spsr(&self, op_mode: OperatingMode) -> Option<Psr> {
        self.get_bank(op_mode)
            .map(|(slice, _)| slice[slice.len() - 1].into())
    }

    pub fn set_spsr(&mut self, op_mode: OperatingMode, cpsr: Psr) {
        if let Some((slice, _)) = self.get_bank_mut(op_mode) {
            slice[slice.len() - 1] = cpsr.value();
        } else {
            unreachable!("invalid operating mode: {op_mode:?}");
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
