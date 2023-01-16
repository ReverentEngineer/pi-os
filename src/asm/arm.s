.equ RPI1_PART_NUM, 2934
.section .text.boot
.global _start
_start:
    // Park all but core "0"
    mrc     p15, 0, r5, c0, c0, 5
    // Check for version of RPi
    mov     r3, #0xFFF
    and     r4, r3, r5, LSR #4
    movt    r7, RPI1_PART_NUM
    cmp     r4, r7
    beq     .clear_bss
    // Check for multi-core on RPi2 and up
    and     r5, r5, #3
    cmp     r5, #0
    bne    parked

.clear_bss:   
    // Clear bss
    ldr     r4, =__bss_start
    ldr     r9, =__bss_end
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    b       2f
1:
    stmia   r4!, {{r5-r8}}
2:
    cmp     r4, r9
    blo     1b

    // Setup stack
    ldr     r5, =_start
    mov     sp, r5
  
    /// Jump to Rust entry point
    b      kmain

parked:
    /// Park not primary cores 
    wfe
    b      parked
