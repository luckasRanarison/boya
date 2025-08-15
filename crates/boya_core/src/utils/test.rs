use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use crate::{arm7tdmi::Arm7tdmi, bus::Bus};

const TEST_MEMORY_SIZE: usize = 8196;

pub fn compile_asm(code: &str) -> io::Result<Vec<u8>> {
    let mut child = Command::new("bash")
        .args(["-c", "cat | fasmarm /dev/stdin >(cat) | tail -n +3"]) // skip logs
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write(code.as_bytes())?; // EOF is signaled after stdin is droped
    }

    child.wait_with_output().map(|output| output.stdout)
}

#[derive(Debug)]
pub struct TestBus {
    memory: [u8; TEST_MEMORY_SIZE],
}

impl Default for TestBus {
    fn default() -> Self {
        Self {
            memory: [0; TEST_MEMORY_SIZE],
        }
    }
}

impl TestBus {
    pub fn load_program(&mut self, bytes: &[u8]) {
        let slice = &mut self.memory[..bytes.len()];
        slice.copy_from_slice(bytes);
    }
}

impl Bus for TestBus {
    fn read_u32(&self, address: u32) -> u32 {
        let address = address as usize;

        let b1 = self.memory[address];
        let b2 = self.memory[address.wrapping_add(1)];
        let b3 = self.memory[address.wrapping_add(2)];
        let b4 = self.memory[address.wrapping_add(3)];

        u32::from_le_bytes([b1, b2, b3, b4])
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let address = address as usize;
        let bytes = value.to_le_bytes();

        self.memory[address] = bytes[0];
        self.memory[address.wrapping_add(1)] = bytes[1];
        self.memory[address.wrapping_add(2)] = bytes[2];
        self.memory[address.wrapping_add(3)] = bytes[2];
    }
}

#[derive(Default, Debug)]
pub struct TestBuilder {
    bus: TestBus,
    thumb: bool,

    mem_assertions: Vec<(u32, u32)>,
    reg_assertions: Vec<(usize, u32)>,
    flag_assertions: Vec<(u32, bool)>,

    code: String,
    bytes: Vec<u8>,
}

impl TestBuilder {
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

    pub fn run(self) {
        self.run_steps(1);
    }

    pub fn run_steps(self, steps: usize) {
        let mut cpu = Arm7tdmi::new(self.bus);
        cpu.update_thumb_state(self.thumb);

        for _ in 0..steps {
            cpu.step();
        }

        let formated_bytes = format_hex_bytes(&self.bytes);

        println!("code: {}", self.code);
        println!("bytes: {formated_bytes}");

        cpu.assert_mem(self.mem_assertions);
        cpu.assert_reg(self.reg_assertions);
        cpu.assert_flag(self.flag_assertions);
    }
}

fn format_hex_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:x}"))
        .collect::<Vec<_>>()
        .join(" ")
}
