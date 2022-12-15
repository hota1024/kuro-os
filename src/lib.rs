#![no_std]
#![feature(slice_ptr_get)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

global_asm!(include_str!("asm/entry.s"));
global_asm!(include_str!("asm/kernelvec.s"));

mod allocator;
mod console;
mod consts;
mod ie;
mod lock;
mod mem;
mod process;
mod riscv;
mod rust_main;
mod start;
