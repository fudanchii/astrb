pub mod access;
pub mod expression;
pub mod literals;

pub trait Emitter {
    fn emit(&self) -> String;
}
