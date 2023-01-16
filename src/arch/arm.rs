use core::arch::{global_asm, asm};
global_asm!(include_str!("asm/arm.s"));

/// Get ARM part num
pub fn part_num() -> u16 {
    #[allow(unused_assignments)]
    let mut midr = 0;
    unsafe {    
        asm!("mrc p15, 0, {0}, c0, c0, 0", out(reg) midr); 
    }
    (midr >> 4) & 0xFFF 
}

/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("3:", "subs {0}, {0}, #1", "bne 3b", inout(reg) count => _, options(nomem, nostack),);
    }
}
