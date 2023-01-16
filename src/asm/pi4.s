.section .text.boot
.global _start
    .org 0x80000
_start:
    mrs	    x0, mpidr_el1
    and     x0, x0, #3
    ldr     x1, =0
    cmp     x0, x1
    b.ne    parked

    ldr     x5, =_start
    mov     sp, x5
    
    // clear bss
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
1:
    cbz     w6, 2f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 1b
 
2:
    b      kmain

parked:
    wfe
    b      parked
