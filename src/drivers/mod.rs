use crate::core::cpu;

#[cfg(any(target_arch = "arm", target_arch ="aarch64"))]
mod bcm2xxx;

pub fn init() {
    #[cfg(any(target_arch = "arm", target_arch ="aarch64"))]
    {
        let midr = cpu::Info::get().manufacturer_id();;
        if midr.implementer() == 0x41 {
            match midr.part_num() {
                0xB76 | 0xC07 | 0xD03 | 0xD08 => {
                    bcm2xxx::init();
                },
                _ => ()
            }
            
        }
    }
}
