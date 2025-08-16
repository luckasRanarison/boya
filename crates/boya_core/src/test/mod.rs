pub mod asm;
pub mod bus;

use asm::{compile_asm, format_bin_bytes, format_hex_bytes};
use bus::TestBus;

use crate::arm7tdmi::{Arm7tdmi, utils::Psr};

#[derive(Default)]
pub struct AsmTestBuilder {
    bus: TestBus,
    thumb: bool,
    code: String,
    bytes: Vec<u8>,
    setup: Option<Box<dyn Fn(&mut Arm7tdmi<TestBus>)>>,

    mem_assertions: Vec<(u32, u32)>,
    reg_assertions: Vec<(usize, u32)>,
    flag_assertions: Vec<(u32, bool)>,
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
        let source = match self.thumb {
            true => format!("code16\n{code}"),
            false => code.to_string(),
        };

        match compile_asm(&source) {
            Ok(bytes) => {
                self.bytes = bytes.clone();
                self.code = code.to_string();
                self.bus.load_program(&bytes)
            }
            Err(err) => panic!("{err}\n\nfailed to compile:\n{code}"),
        }

        self
    }

    pub fn assert_mem(mut self, address: u32, expected: u32) -> Self {
        self.mem_assertions.push((address, expected));
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

    pub fn setup<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut Arm7tdmi<TestBus>) + 'static,
    {
        self.setup = Some(Box::new(func));
        self
    }

    pub fn run(self, steps: usize) {
        let formated_bits = format_bin_bytes(&self.bytes);
        let formated_bytes = format_hex_bytes(&self.bytes);

        println!("code: {}", self.code);
        println!("hex: {formated_bytes}");
        println!("bin: {formated_bits}");

        let mut cpu = match self.thumb {
            true => Arm7tdmi::new_thumb(self.bus),
            false => Arm7tdmi::new(self.bus),
        };

        if let Some(setup) = self.setup {
            setup(&mut cpu);
        }

        for _ in 0..steps {
            cpu.step();
        }

        cpu.assert_mem(self.mem_assertions);
        cpu.assert_reg(self.reg_assertions);
        cpu.assert_flag(self.flag_assertions);
    }
}
