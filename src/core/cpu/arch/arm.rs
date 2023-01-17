
#[derive(Copy, Clone, Debug)]
pub struct ManufacturerID {
    implementer: u8,
    variant: u8,
    architecture: u8,
    part_num: u16,
    revision: u8
}

impl ManufacturerID {

    pub fn from_midr(midr: u32) -> Self {
        Self {
            implementer: ((midr >> 24) & 0xFF) as u8,
            variant: ((midr >> 20) & 0x0F) as u8,
            architecture: ((midr >> 16) & 0x0F) as u8,
            part_num: ((midr >> 4) & 0xFFF) as u16,
            revision: (midr & 0x0F) as u8
        }
    }

    /// Implementer of the CPU
    pub fn implementer(&self) -> u8 {
        self.implementer
    }

    /// Variant of the CPU
    pub fn variant(&self) -> u8 {
        self.variant
    }

    /// ARM-architecture specificiation
    pub fn architecture(&self) -> u8 {
        self.architecture
    }

    /// Part number
    pub fn part_num(&self) -> u16 {
        self.part_num
    }


    /// Revision of part
    pub fn revision(&self) -> u8 {
        self.revision
    }

}
