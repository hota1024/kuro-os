pub const UART0: usize = 0x10000000;

pub const CLINT: usize = 0x02000000;
pub const CLINT_MTIMECMP: usize = CLINT + 0x4000;
pub const CLINT_MTIME: usize = CLINT + 0xbff8;

pub const KERNEL_BASE: usize = 0x80000000;
pub const PHYSTOP: usize = KERNEL_BASE + 128 * 1024 * 1024;
