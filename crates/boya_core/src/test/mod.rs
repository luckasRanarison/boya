mod asm;

use crate::{
    Gba,
    arm7tdmi::{Arm7tdmi, psr::Psr},
    bus::{BIOS_SIZE, Bus, GbaBus, types::DataType},
    test::asm::FAKE_BIOS,
};

use asm::{compile_asm, format_bin_bytes, format_hex_bytes};

pub const SP_START: u32 = 0x0300_7F00;
pub const ARM_MAIN_START: u32 = 0x0800_0000;
pub const TMB_MAIN_START: u32 = 0x0800_0012;

#[derive(Default)]
pub struct AsmTestBuilder {
    thumb: bool,
    code: String,
    bytes: Vec<u8>,
    setup: Option<Box<dyn Fn(&mut Arm7tdmi)>>,
    pc: Option<u32>,

    flag_assertions: Vec<(u32, bool)>,
    reg_assertions: Vec<(usize, u32)>,
    mem_assertions: Vec<(u32, u32, DataType)>,
    func_assertion: Option<Box<dyn Fn(&Arm7tdmi)>>,
    cycle_assertions: Vec<u32>,
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

    pub fn assert_cycles<I: IntoIterator<Item = u32>>(mut self, cycles: I) -> Self {
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

    pub fn run_while<F>(&self, mut func: F)
    where
        F: FnMut(&Arm7tdmi) -> bool + 'static,
    {
        self.debug_run();

        let mut gba = self.init_gba();
        let mut cycles = Vec::new();

        while func(&gba.cpu) {
            self.debug_instruction(&gba.cpu);

            let cycle = gba.step();

            cycles.push(cycle.count());
        }

        self.run_assertions(&gba, cycles.as_slice());
    }

    fn debug_run(&self) {
        let formated_bits = format_bin_bytes(&self.bytes);
        let formated_bytes = format_hex_bytes(&self.bytes);

        println!("code: {}", self.code);
        println!("hex: {formated_bytes}");
        println!("bin: {formated_bits}\n");
    }

    fn debug_instruction(&self, cpu: &Arm7tdmi) {
        println!(
            "{:#08x}: {:?}",
            cpu.pipeline.curr_pc,
            cpu.pipeline.curr_instr.as_ref().unwrap(),
        );
        // println!("{:?}", cpu.cpsr);
    }

    fn run_assertions(&self, gba: &Gba, cycles: &[u32]) {
        if !self.cycle_assertions.is_empty() {
            assert_eq!(&self.cycle_assertions, cycles);
        }

        if let Some(assert) = &self.func_assertion {
            assert(&gba.cpu);
        }

        self.assert_mem(&gba.cpu, &self.mem_assertions);
        self.assert_reg_impl(&gba.cpu, &self.reg_assertions);
        self.assert_flag_impl(&gba.cpu, &self.flag_assertions);
    }

    fn init_bios(&self) -> [u8; BIOS_SIZE] {
        let mut bios = [0; BIOS_SIZE];
        let fake_bios = compile_asm(FAKE_BIOS).unwrap();

        for (i, byte) in fake_bios.iter().enumerate() {
            bios[i] = *byte;
        }

        bios
    }

    fn init_rom(&self) -> Vec<u8> {
        // Make test ROMs bigger to avoid out of bound indexing
        let rom_size = usize::max(self.bytes.len(), 1024);
        let mut rom = vec![0; rom_size];
        let rom_slice = &mut rom[..self.bytes.len()];

        rom_slice.copy_from_slice(&self.bytes);
        rom
    }

    fn init_bus(&self) -> GbaBus {
        let mut bus = GbaBus::default();

        let rom = self.init_rom();
        let bios = self.init_bios();

        bus.load_bios(&bios);
        bus.load_rom(&rom);

        bus
    }

    fn init_gba(&self) -> Gba {
        let bus = self.init_bus();
        let mut cpu = Arm7tdmi::new(bus);

        cpu.reset();

        let extra_steps = if self.thumb { 9 } else { 4 };

        for _ in 0..extra_steps {
            cpu.step();
        }

        if let Some(setup) = &self.setup {
            setup(&mut cpu);
        }

        if let Some(pc) = self.pc {
            cpu.override_pc(pc);
        }

        Gba { cpu }
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

    pub fn assert_mem(&self, cpu: &Arm7tdmi, assertions: &[(u32, u32, DataType)]) {
        for (address, expected, data_type) in assertions {
            let value = match data_type {
                DataType::Byte => cpu.bus.read_byte(*address).into(),
                DataType::HWord => cpu.bus.read_hword(*address).into(),
                DataType::Word => cpu.bus.read_word(*address),
            };

            assert_eq!(
                value, *expected,
                "expected {expected:#x} at {address:#x}, got {value:#x}"
            )
        }
    }

    pub fn assert_reg_impl(&self, cpu: &Arm7tdmi, assertions: &[(usize, u32)]) {
        for (index, expected) in assertions {
            let value = cpu.get_reg(*index);

            assert_eq!(
                value, *expected,
                "expected {expected:#x} at R{index}, got {value:#x}"
            )
        }
    }

    pub fn assert_flag_impl(&self, cpu: &Arm7tdmi, assertions: &[(u32, bool)]) {
        for (flag, expected) in assertions {
            let value = cpu.cpsr.has(*flag);
            let name = Psr::format_flag(*flag);
            let status = if *expected { "set" } else { "cleared" };

            assert_eq!(
                value, *expected,
                "expected flag {name} to be {status}, flags: {:?}",
                cpu.cpsr
            )
        }
    }
}
