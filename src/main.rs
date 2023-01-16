#![no_std]
#![no_main]

use core::arch::{global_asm, asm};
use core::panic::PanicInfo;

global_asm!(include_str!("asm/aarch64.s"));

/// Delay execution 
pub fn delay(count: u32) {
    unsafe {
        asm!("1:", "subs {0:w}, {0:w}, #1", "bne 1b", inout(reg) count => _, options(nomem, nostack),);
    }
}

pub fn pi_version() -> usize {
    #[allow(unused_assignments)]
    let mut midr_el = 0;
    unsafe {
        asm!("mrs {0:x}, midr_el1", out(reg) midr_el); 
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
