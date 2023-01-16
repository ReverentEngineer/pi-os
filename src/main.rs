#![no_std]
#![no_main]

use ::core::panic::PanicInfo;

mod arch;
mod drivers;
mod mmu;
mod interrupts;
mod core;

#[export_name = "kmain"]
pub extern "C" fn kmain() {
    core::log::init();
    drivers::init();
    ::log::info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    ::log::info!("-------------------------");
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
