#![feature(naked_functions)]
pub mod stack;
mod x86_64;
pub use x86_64::{it, switch, Context};
