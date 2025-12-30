mod asm;

use std::collections::VecDeque;

use crate::{
    arm7tdmi::Arm7tdmi,
    bus::{BIOS_SIZE, GbaBus, types::DataType},
    test::asm::FAKE_BIOS,
};

use asm::{compile_asm, format_bin_bytes, format_hex_bytes};

pub const SP_START: u32 = 0x0300_7F00;
pub const ARM_MAIN_START: u32 = 0x0800_0000;
pub const TMB_MAIN_START: u32 = 0x0800_0012;

#[derive(Default)]
pub struct AsmTestBuilder {
    bus: GbaBus,
    thumb: bool,
    code: String,
    bytes: Vec<u8>,
    setup: Option<Box<dyn Fn(&mut Arm7tdmi)>>,
    pc: Option<u32>,

    flag_assertions: Vec<(u32, bool)>,
    reg_assertions: Vec<(usize, u32)>,
    mem_assertions: Vec<(u32, u32, DataType)>,
    func_assertion: Option<Box<dyn Fn(&Arm7tdmi)>>,
    cycle_assertions: VecDeque<u8>,
}

impl AsmTestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn thumb(mut self) -> Self {
        self.thumb = true;
        self
    }

    pub fn asm(mut self, code: &str) -> Self {
        let source = if self.thumb {
            self.make_thumb_code(code)
        } else {
            code.to_string()
        };

        match compile_asm(&source) {
            Ok(bytes) => {
                self.bytes = bytes.clone();
                self.code = code.to_string();
            }
            Err(err) => panic!("{err}\n\nfailed to compile:\n{code}"),
        }

        self
    }

    pub fn bytes(mut self, bytes: &[u8]) -> Self {
        self.bytes = bytes.to_vec();
        self
    }

    pub fn pc(mut self, value: u32) -> Self {
        self.pc = Some(value);
        self
    }

    pub fn assert_byte(mut self, addr: u32, expected: u32) -> Self {
        self.mem_assertions.push((addr, expected, DataType::Byte));
        self
    }

    pub fn assert_hword(mut self, addr: u32, expected: u32) -> Self {
        self.mem_assertions.push((addr, expected, DataType::HWord));
        self
    }

    pub fn assert_word(mut self, addr: u32, expected: u32) -> Self {
        self.mem_assertions.push((addr, expected, DataType::Word));
        self
    }

    pub fn assert_reg(mut self, index: usize, expected: u32) -> Self {
        self.reg_assertions.push((index, expected));
        self
    }

    pub fn assert_flag(mut self, flag: u32, status: bool) -> Self {
        self.flag_assertions.push((flag, status));
        self
    }

    pub fn assert_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&Arm7tdmi) + 'static,
    {
        self.func_assertion = Some(Box::new(func));
        self
    }

    pub fn assert_cycles<I: IntoIterator<Item = u8>>(mut self, cycles: I) -> Self {
        self.cycle_assertions = cycles.into_iter().collect();
        self
    }

    pub fn setup<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut Arm7tdmi) + 'static,
    {
        self.setup = Some(Box::new(func));
        self
    }

    pub fn run(self, steps: usize) {
        let mut i = 0;

        self.run_while(move |_| {
            i += 1;
            i <= steps
        });
    }

    pub fn run_while<F>(mut self, mut func: F)
    where
        F: FnMut(&Arm7tdmi) -> bool + 'static,
    {
        let formated_bits = format_bin_bytes(&self.bytes);
        let formated_bytes = format_hex_bytes(&self.bytes);

        println!("code: {}", self.code);
        println!("hex: {formated_bytes}");
        println!("bin: {formated_bits}\n");

        let mut bios = [0; BIOS_SIZE];
        let fake_bios = compile_asm(FAKE_BIOS).unwrap();

        for (i, byte) in fake_bios.iter().enumerate() {
            bios[i] = *byte;
        }

        self.bus.load_bios(&bios);
        self.bus.load_rom(&self.bytes);

        let mut cpu = Arm7tdmi::new(self.bus);

        cpu.reset();

        let extra_steps = if self.thumb { 9 } else { 4 };

        for _ in 0..extra_steps {
            cpu.step();
        }

        if let Some(setup) = self.setup {
            setup(&mut cpu);
        }

        if let Some(pc) = self.pc {
            cpu.override_pc(pc);
        }

        while func(&cpu) {
            // println!("{:?}", cpu.cpsr);
            println!(
                "{:#08x}: {:?}",
                cpu.pipeline.curr_pc,
                cpu.pipeline.curr_instr.as_ref().unwrap(),
            );

            let cycles = cpu.step();

            if let Some(expected_cycles) = self.cycle_assertions.pop_front() {
                assert_eq!(
                    cycles.count(),
                    expected_cycles,
                    "instruction cycle mismatch, expected: {expected_cycles}"
                )
            }
        }

        if let Some(assert) = self.func_assertion {
            assert(&cpu);
        }

        cpu.assert_mem(self.mem_assertions);
        cpu.assert_reg(self.reg_assertions);
        cpu.assert_flag(self.flag_assertions);
    }

    fn make_thumb_code(&self, code: &str) -> String {
        format!(
            r"
            _setup:
                MOV     R0, 0x0800_0000
                ADD     R0, R0, 0x10
                ORR     R0, R0, #1 ; set bit 1
                BX      R0         ; switch to thumb mode

            code16
            _main:
                mov     r0, 0      ; reset r0
                {code}
            "
        )
    }
}
