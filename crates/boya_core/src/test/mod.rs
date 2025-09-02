mod asm;
mod bus;

use asm::{format_bin_bytes, format_hex_bytes};
use bus::TestBus;

use crate::{
    arm7tdmi::{Arm7tdmi, test::DataType},
    bus::Bus,
};

pub use asm::compile_asm;

pub const SP_START: u32 = 0x20;
pub const PRG_START: u32 = 0x200;
pub const ARM_MAIN_START: u32 = 0x200;
pub const TMB_MAIN_START: u32 = 0x20E;

type CpuFn = Box<dyn Fn(&Arm7tdmi<TestBus>)>;
type CpuFnMut = Box<dyn Fn(&mut Arm7tdmi<TestBus>)>;

#[derive(Default)]
pub struct AsmTestBuilder {
    bus: TestBus,
    thumb: bool,
    code: String,
    bytes: Vec<u8>,
    setup: Option<CpuFnMut>,

    flag_assertions: Vec<(u32, bool)>,
    reg_assertions: Vec<(usize, u32)>,
    mem_assertions: Vec<(u32, u32, DataType)>,
    func_assertion: Option<CpuFn>,
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
        F: Fn(&Arm7tdmi<TestBus>) + 'static,
    {
        self.func_assertion = Some(Box::new(func));
        self
    }

    pub fn setup<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut Arm7tdmi<TestBus>) + 'static,
    {
        self.setup = Some(Box::new(func));
        self
    }

    pub fn run(mut self, steps: usize) {
        let formated_bits = format_bin_bytes(&self.bytes);
        let formated_bytes = format_hex_bytes(&self.bytes);

        println!("code: {}", self.code);
        println!("hex: {formated_bytes}");
        println!("bin: {formated_bits}\n");

        self.bus.write_word(0x0, 0xEA00007E); // reset: branch to 0x200
        self.bus.load_program(&self.bytes, PRG_START as usize);

        let mut cpu = Arm7tdmi::new(self.bus);

        cpu.reset();
        cpu.set_sp(SP_START);

        if let Some(setup) = self.setup {
            setup(&mut cpu);
        }

        let extra_cycle = if self.thumb { 5 } else { 1 };

        for _ in 0..extra_cycle {
            cpu.step();
        }

        for _ in 0..steps {
            if let Some(instr) = &cpu.pipeline.curr_instr {
                println!("{:#08x}: {instr:?}", cpu.pipeline.curr_pc);
            }

            cpu.step();
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
                MOV     R0, 0x20C
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
