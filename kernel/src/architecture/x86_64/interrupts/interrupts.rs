use crate::architecture::x86_64::interrupts::interrupt_descriptor_table;

pub fn initialize() {
    log::info!("Initializing Interrupt Descriptor Table");
    interrupt_descriptor_table::initialize();
}
