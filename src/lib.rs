#![no_std]
use core::arch::global_asm;

global_asm!(include_str!("asm/entry.s"));
global_asm!(include_str!("asm/kernelvec.s"));

mod console;
mod consts;
mod ie;
mod riscv;
mod rust_main;
mod start;
