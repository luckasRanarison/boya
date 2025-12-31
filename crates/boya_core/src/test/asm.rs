use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

pub fn compile_asm(code: &str) -> io::Result<Vec<u8>> {
    let mut child = Command::new("bash")
        .args(["-c", "cat | fasmarm /dev/stdin -"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?; // EOF is signaled after stdin is droped
    }

    let output = child.wait_with_output()?;

    if !output.stderr.is_empty() {
        return Err(io::Error::other(String::from_utf8_lossy(&output.stderr)));
    }

    if output.stdout.is_empty() {
        return Err(io::Error::other("empty stream"));
    }

    Ok(output.stdout)
}

pub fn format_hex_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn format_bin_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:08b}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub const FAKE_BIOS: &str = r"
    vectors:
        B       reset_handler
        dw      0x0000_0000 ; UNDEFINED
        dw      0x0000_0000 ; SWI
        dw      0x0000_0000 ; PREFETCH_ABORT
        dw      0x0000_0000 ; DATA_ABORT
        dw      0x0000_0000 ; RESERVED
        dw      0x0000_0000 ; IRQ
        dw      0x0000_0000 ; FIQ

    reset_handler:
        MOV     SP, 0x0300_0000
        ADD     SP, SP, 0x0000_7F00
        MOV     PC, 0x0800_0000
";
