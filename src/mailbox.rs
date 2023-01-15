use crate::mmio; 

bitflags::bitflags! {

    pub struct Status: u32 {
        const FULL = 0x80000000;
        const EMPTY = 0x40000000;
    }

}

pub fn status() -> Status {
    unsafe { Status::from_bits_unchecked(mmio::Register::MBOX_STATUS.read()) }
}


#[derive(serde::Serialize)]
pub enum Tag {
    SetClockRate {
        clock_id: u32, //
        rate: u32, // Hz
        skip_setting_turbo: u32
    }
}
