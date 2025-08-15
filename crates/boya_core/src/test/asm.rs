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
        stdin.write(code.as_bytes())?; // EOF is signaled after stdin is droped
    }

    let output = child.wait_with_output()?;

    if output.stdout.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "empty stream"));
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
