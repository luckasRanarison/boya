#[derive(Debug)]
pub struct Bias {
    pub value: u16,
}

impl Default for Bias {
    fn default() -> Self {
        Self { value: 0x200 }
    }
}
