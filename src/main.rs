#![no_std]
#![no_main]

use core::arch::{global_asm, asm};
use core::panic::PanicInfo;

global_asm!(include_str!("asm/pi4.s"));

/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("1:", "subs {0:w}, {0:w}, #1", "bne 1b", inout(reg) count => _, options(nomem, nostack),);
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
pub extern "C" fn kmain(dtb_ptr32: u64, x1: u64, x2: u64, x3: u64) {
    uart::init();
    log::init();
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
