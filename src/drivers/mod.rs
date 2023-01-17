#[cfg(any(target_arch = "arm", target_arch ="aarch64"))]
mod bcm2xxx;

pub fn init() {
    #[cfg(any(target_arch = "arm", target_arch ="aarch64"))]
    {
        let midr = crate::arch::midr();
        if midr.implementer() == 0x43 {
            match midr.part_num() {
                0xB76 | 0xC07 | 0xD03 | 0xD08 => {
                    bcm2xxx::init();
                },
                _ => ()
            }
            
        }
    }
}
