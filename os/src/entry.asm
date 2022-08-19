    .section .text.entry
    .global _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 300
    .globl boot_stack_top
boot_stack_top: