#![feature(no_core)]
#![no_core]
mod inner { pub struct Public; }
pub use inner::Public;