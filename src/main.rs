#![no_std]
#![no_main]

use core::arch::{global_asm, asm};
use core::panic::PanicInfo;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64.s"));

#[cfg(target_arch = "arm")]
global_asm!(include_str!("asm/arm.s"));

#[cfg(target_arch = "aarch64")]
/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("1:", "subs {0:w}, {0:w}, #1", "bne 1b", inout(reg) count => _, options(nomem, nostack),);
    }
}

#[cfg(target_arch = "arm")]
/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("3:", "subs {0}, {0}, #1", "bne 3b", inout(reg) count => _, options(nomem, nostack),);
    }
}

pub fn pi_version() -> usize {
    #[allow(unused_assignments)]
    let mut midr_el = 0;
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("mrs {0:x}, midr_el1", out(reg) midr_el); 
        #[cfg(target_arch = "arm")]
        asm!("mrc p15, 0, {0}, c0, c0, 0", out(reg) midr_el); 
    }
    match (midr_el >> 4) & 0xFFF {
        0xB76 => 1,
        0xC07 => 2,
        0xD03 => 3,
        0xD08 => 4,
        _ => panic!("Unexpected part number")
    }
}

mod sync;
mod mmu;
mod mmio;
mod mailbox;
mod console;
mod uart;
mod log;
mod interrupts;

#[export_name = "kmain"]
pub extern "C" fn kmain() {
    let pi_version = pi_version();
    mmio::init(pi_version);
    uart::init();
    console::set(uart::get());
    log::init();
    ::log::info!("Raspberry Pi {pi_version}");
    ::log::info!("UART initialized.");
    mmu::init();
    loop {
        //uart::UART.write_char(uart::UART.read_char());
    }
}

use ::log::error;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");
    loop {}
}
