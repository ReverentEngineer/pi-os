use core::arch::{global_asm, asm};
use super::arm::ManufacturerID;
global_asm!(include_str!("asm/aarch64.s"));

pub fn midr() -> ManufacturerID {
    let mut midr = 0;
    unsafe {    
        asm!("mrs {0:x}, midr_el1", out(reg) midr);
    }
    ManufacturerID::from_midr(midr as u32)
}

/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("1:", "subs {0:w}, {0:w}, #1", "bne 1b", inout(reg) count => _, options(nomem, nostack),);
    }
}
