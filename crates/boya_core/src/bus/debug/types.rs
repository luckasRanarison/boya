#[derive(Debug, Clone, Copy)]
pub enum RegisterSize {
    HWord,
    Word,
}

#[derive(Debug)]
pub struct RegisterEntry {
    pub name: &'static str,
    pub address: u32,
    pub size: RegisterSize,
    pub flags: &'static [Flag],
}

#[derive(Debug)]
pub struct Flag {
    pub name: &'static str,
    pub start: u8,
    pub length: u8,
    pub mappings: Option<&'static [(u8, &'static str)]>,
}

impl Flag {
    pub const fn new(name: &'static str, start: u8, length: u8) -> Self {
        Self {
            name,
            start,
            length,
            mappings: None,
        }
    }

    pub const fn unused(start: u8, length: u8) -> Self {
        Self {
            name: "unused",
            start,
            length,
            mappings: None,
        }
    }

    pub const fn map(mut self, mappings: &'static [(u8, &'static str)]) -> Self {
        self.mappings = Some(mappings);
        self
    }
}
