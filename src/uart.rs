use core::arch::asm;
use crate::mmio::{self, MMIORegister};
use crate::delay;
use crate::mailbox;

pub static MBOX: [u32; 9] = [
    9*4, 0, 0x38002, 12, 8, 2, 3000000, 0, 0 	
];

/// Initialize UART
pub fn init() {
	mmio::write(MMIORegister::UART0_CR, 0);
	mmio::write(MMIORegister::GPPUD, 0);
	delay(300);
	mmio::write(MMIORegister::GPPUDCLK0, (1 << 14) | (1 << 15));
	delay(300);
	mmio::write(MMIORegister::GPPUDCLK0, 0);
	mmio::write(MMIORegister::UART0_ICR, 0x7FF);

	// For RPi3/4 only
	let r = (MBOX.as_ptr() as u32  & !0xF) | 8;
        while mailbox::status().contains(mailbox::Status::FULL) { unsafe { asm!("nop") } }
        mmio::write(MMIORegister::MBOX_WRITE, r);
        while mailbox::status().contains(mailbox::Status::EMPTY) ||
            mmio::read(MMIORegister::MBOX_READ) != r { unsafe { asm!("nop") } }

	mmio::write(MMIORegister::UART0_IBRD, 1);
	mmio::write(MMIORegister::UART0_FBRD, 40);
	mmio::write(MMIORegister::UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));
	mmio::write(MMIORegister::UART0_IMSC, (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
	                       (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));
 
	mmio::write(MMIORegister::UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
}

/// Write character to UART
pub fn putc(c: u8) {
    while mmio::read(MMIORegister::UART0_FR) & (1 << 5) == (1 << 5) { }
    mmio::write(MMIORegister::UART0_DR, c as u32);
}

/// Get character from UART
pub fn getc() -> u8 {
    while mmio::read(MMIORegister::UART0_FR) & (1 << 5) == (1 << 5) { }
    return mmio::read(MMIORegister::UART0_DR) as u8;
}

/// Put a string of character to UART
pub fn puts(s: &[u8]) {
    for c in s {
        putc(*c);
    }
}
