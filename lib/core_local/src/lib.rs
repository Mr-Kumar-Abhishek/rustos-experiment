#![no_std]
#![feature(box_syntax, asm, unsafe_destructor)]
#![feature(core, alloc, collections)]
#![feature(no_std)]

extern crate core;
extern crate collections;
extern crate alloc;

pub mod task_queue;

pub unsafe fn init() {
    task_queue::init();
}