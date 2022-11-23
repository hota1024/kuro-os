#![no_std]
use core::arch::global_asm;

global_asm!(include_str!("asm/entry.s"));

mod console;
mod consts;
mod riscv;
mod rust_main;
mod start;
