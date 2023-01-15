use crate::mmu;

/// Base memory-mapped IO address
const MMIO_BASE: u32 = 0x3F000000;

/// Memory-mapped IO registers
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Register {
    GPPUD        = 0x200094,
    GPPUDCLK0    = 0x20009C,

    UART0_DR     = 0x201000,
    UART0_RSRECR = 0x201004,
    UART0_FR     = 0x201018,
    UART0_ILPR   = 0x201020,
    UART0_IBRD   = 0x201024,
    UART0_FBRD   = 0x201028,
    UART0_LCRH   = 0x20102C,
    UART0_CR     = 0x201030,
    UART0_IFLS   = 0x201034,
    UART0_IMSC   = 0x201038,
    UART0_RIS    = 0x20103C,
    UART0_MIS    = 0x201040,
    UART0_ICR    = 0x201044,
    UART0_DMACR  = 0x201048,
    UART0_ITCR   = 0x201080,
    UART0_ITIP   = 0x201084,
    UART0_ITOP   = 0x201088,
    UART0_TDR    = 0x20108C,

    MBOX_READ    = 0x00BB80,
    MBOX_STATUS  = 0x00BB98,
    MBOX_WRITE   = 0x00BBa0,
}

impl Register {

    /// Write to a memory-mapped I/O register
    pub fn write(&self,  data: u32) {
        let address = (MMIO_BASE + *self as u32) as *mut u32;
        unsafe { mmu::write(address, data) };
    }

    /// Read from a memory-mapped I/O register
    pub fn read(&self) -> u32
    {
        let address = (MMIO_BASE + *self as u32) as *mut u32;
        unsafe { mmu::read(address) }
    }

}
