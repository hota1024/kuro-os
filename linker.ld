OUTPUT_ARCH(riscv)

ENTRY(_entry)
EXTERN(abort);

MEMORY {
    DRAM (xw): ORIGIN = 0x80000000, LENGTH = 100K
}

SECTIONS {
    . = 0x80000000;
    .reset : {
        KEEP(*(.entry));
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

    .bss : {
        *(.bss .bss.*);
    } > DRAM

    PROVIDE(end = .)
}
