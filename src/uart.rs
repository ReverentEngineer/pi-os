use core::arch::asm;
use core::fmt::{self, Write};
use core::ops::DerefMut;
use crate::{
    mmio,
    delay,
    mailbox,
    sync::RawSpinlock
};
use crate::console;
use lock_api::Mutex;

pub static MBOX: [u32; 9] = [
    9*4, 0, 0x38002, 12, 8, 2, 3000000, 0, 0 	
];

static UART: Uart = Uart::new();

/// Representation of the UART
pub struct Uart {
    inner: Mutex<RawSpinlock, UartInner> 
}

struct UartInner;

impl fmt::Write for UartInner {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            while mmio::Register::UART0_FR.read()  & (1 << 5) == (1 << 5) { }
            mmio::Register::UART0_DR.write(*c as u32);
        }
        Ok(())
    }
}

impl Uart {

    const fn new() -> Self {
        Uart { inner: Mutex::new(UartInner) }
    }

}

impl console::Read for Uart { 

    fn read_char(&self) -> char {
        while mmio::Register::UART0_FR.read()  & (1 << 5) == (1 << 5) { }
        return unsafe { char::from_u32_unchecked(mmio::Register::UART0_DR.read()) };
    }

}

impl console::Write for Uart {
    fn write(&self, args: fmt::Arguments) -> fmt::Result {
        let mut guard = self.inner.lock();
        (*guard).write_fmt(args)
    }

    fn flush(&self) {
        while mmio::Register::UART0_FR.read()  & (1 << 3) == (1 << 3) { }
    }
}

pub fn get() -> &'static Uart {
    &UART
}

/// Initialize UART
pub fn init() {
	mmio::Register::UART0_CR.write(0);
	mmio::Register::GPPUD.write(0);
	delay(300);
	mmio::Register::GPPUDCLK0.write((1 << 14) | (1 << 15));
	delay(300);
	mmio::Register::GPPUDCLK0.write(0);
	mmio::Register::UART0_ICR.write(0x7ff);

	// For RPi3/4 only
	let r = (MBOX.as_ptr() as u32  & !0xF) | 8;
        while mailbox::status().contains(mailbox::Status::FULL) { unsafe { asm!("nop") } }
        mmio::Register::MBOX_WRITE.write(r);
        while mailbox::status().contains(mailbox::Status::EMPTY) ||
            mmio::Register::MBOX_READ.read() != r { unsafe { asm!("nop") } }

	mmio::Register::UART0_IBRD.write(1);
	mmio::Register::UART0_FBRD.write(40);
	mmio::Register::UART0_LCRH.write((1 << 4) | (1 << 5) | (1 << 6));
	mmio::Register::UART0_IMSC.write((1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
	                       (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));
 
	mmio::Register::UART0_CR.write((1 << 0) | (1 << 8) | (1 << 9));
}
