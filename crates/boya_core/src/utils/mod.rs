pub mod bitflags;
pub mod ops;

pub trait Reset {
    fn reset(&mut self);
}
