.section .text
.global _entry

_entry:
    /* スタックの確保 */
    la sp, stacks + 1024

    /* start 関数の呼び出し */
    call start

.bss

stacks:
    .space 4096 * 8 /* 8 cpus */

