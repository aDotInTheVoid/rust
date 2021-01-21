#![no_core]
#![feature(no_core)]
mod inner {
    pub struct Public;
}
pub use inner::Public as Reexported;