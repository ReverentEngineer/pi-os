#[repr(packed)]
struct RPIIRQController {
    irq_basic_pending: u32,
    irq_pending_1: u32,
    irq_pending_2: u32,
    fiq_control: u32,
    enable_irqs_1: u32,
    enable_irqs_2: u32,
    enable_basic_irqs: u32,
    disable_irqs_1: u32,
    disable_irqs_2: u32,
    disable_basic_irqs: u32,
}

const RPI_PERIPHERAL_BASE: u32 = 0xFE000000u32;
const RPI4_GIC400_BASE: u32 = 0xFF840000u32;

const RPI_INTERRUPT_CONTROLLER_BASE: u32 = RPI_PERIPHERAL_BASE + 0xB200u32;

use bitflags::bitflags;

bitflags! {

    pub struct IRQ: u32 {
        const SYSTEM_TIMER_IRQ_0 = 0b00000001;
        const SYSTEM_TIMER_IRQ_1 = 0b00000010;
        const SYSTEM_TIMER_IRQ_2 = 0b00000100;
        const SYSTEM_TIMER_IRQ_3 = 0b00001000;
    }

}

pub(crate) fn enable_interrupts() {
    let mut irq_contoller = RPI_INTERRUPT_CONTROLLER_BASE as *mut RPIIRQController; 
    unsafe { (*irq_contoller).enable_irqs_1 = IRQ::SYSTEM_TIMER_IRQ_1.bits() };
}
