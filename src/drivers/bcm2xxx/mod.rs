mod mailbox;
mod mmio;
mod uart;
use crate::core::cpu;

pub fn pi_version() -> usize {
    match cpu::Info::get().manufacturer_id().part_num() {
        0xB76 => 1,
        0xC07 => 2,
        0xD03 => 3,
        0xD08 => 4,
        _ => panic!("Unexpected part number")
    }
}


pub fn init() {
    let pi_version = pi_version();
    mmio::init(pi_version);
    uart::init();
    crate::core::console::set(uart::get());
}
