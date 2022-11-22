OUTPUT_ARCH(riscv)

ENTRY(_start)
EXTERN(abort);

MEMORY {
    DRAM (xw): ORIGIN = 0x80000000, LENGTH = 100K
}

SECTIONS {
    . = 0x80000000;
    .reset : {
        KEEP(*(.boot));
    } > DRAM

    .text : {
        *(.text. text.*);
    } > DRAM

    .rodata : {
        *(.rdata .rodata. rodata.*);
    } > DRAM

    .data : {
        *(.data .data.*);
    } > DRAM

    _bss_start = .;
    .bss : {
        *(.bss .bss.*);
    } > DRAM
}
