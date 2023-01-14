use crate::mmio::{self, MMIORegister}; 

bitflags::bitflags! {

    pub struct Status: u32 {
        const FULL = 0x80000000;
        const EMPTY = 0x40000000;
    }

}

pub fn status() -> Status {
    unsafe { Status::from_bits_unchecked(mmio::read(MMIORegister::MBOX_STATUS)) }
}

