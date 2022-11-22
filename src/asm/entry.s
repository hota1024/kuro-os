.section .text
.global _entry

_entry:
    la      sp, stacks + 1024
    call    start

.bss

stacks:
    .space 4096 * 8

