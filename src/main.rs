#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub fn pi_version() -> usize {
    match arch::part_num() {
        0xB76 => 1,
        0xC07 => 2,
        0xD03 => 3,
        0xD08 => 4,
        _ => panic!("Unexpected part number")
    }
}

mod arch;
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
    ::log::info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    ::log::info!("-------------------------");
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
