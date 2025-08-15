use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

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

pub fn format_hex_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:x}"))
        .collect::<Vec<_>>()
        .join(" ")
}
