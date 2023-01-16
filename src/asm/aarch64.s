.section .text.boot
.global _start
    .org 0x80000
_start:
    // Park all but core "0"
    mrs	    x0, mpidr_el1
    and     x0, x0, #3
    ldr     x1, =0
    cmp     x0, x1
    b.ne    parked

   
    // Clear bss
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
1:
    cbz     w6, 2f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 1b
 
2:
    // Setup stack
    ldr     x5, =_start
    mov     sp, x5
  
    /// Jump to Rust entry point
    b      kmain

parked:
    /// Park not primary cores 
    wfe
    b      parked
