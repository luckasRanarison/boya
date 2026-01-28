pub mod bitflags;
pub mod collections;
pub mod ops;

pub trait Reset {
    fn reset(&mut self);
}
