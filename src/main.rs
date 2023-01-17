#![no_std]
#![no_main]

use ::core::panic::PanicInfo;
use ::log::{error, info};

mod drivers;
mod mmu;
mod interrupts;
mod core;

#[export_name = "kmain"]
pub extern "C" fn kmain() {
    core::log::init();
    drivers::init();
    info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    info!("-------------------------");
    mmu::init();
    loop {
        //uart::UART.write_char(uart::UART.read_char());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");
    loop {}
}
