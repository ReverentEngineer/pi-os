use core::arch::{global_asm, asm};
global_asm!(include_str!("asm/aarch64.s"));

/// Get ARM part num
pub fn part_num() -> u16 {
    #[allow(unused_assignments)]
    let mut midr = 0;
    unsafe {    
        asm!("mrs {0:x}, midr_el1", out(reg) midr); 
    }
    (midr >> 4) & 0xFFF 
}

/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("1:", "subs {0:w}, {0:w}, #1", "bne 1b", inout(reg) count => _, options(nomem, nostack),);
    }
}
