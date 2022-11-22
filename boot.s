.option norvc

.section .boot, "ax", @progbits

.global _start

_start:
    la      sp, stacks + 1024
    j       __start_rust

.bss

stacks:
    .skip 1024

