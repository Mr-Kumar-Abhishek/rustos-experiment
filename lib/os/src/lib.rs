#![feature(unsafe_destructor, box_syntax)]
#![feature(core)]
#![allow(dead_code)]

extern crate core_local;
pub mod async;

// compatibility (we use parts of libstd)
mod core {
    pub use std::{cell, mem, ptr, prelude};
}
mod alloc {
    pub use std::boxed;
}
mod sync {
    pub use std::sync::atomic;
}