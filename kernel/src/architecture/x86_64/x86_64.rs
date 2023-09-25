use core::pin::Pin;

use crate::architecture::architecture_api::ArchitectureApi;
use crate::architecture::x86_64::interrupts::InterruptDescriptorTable;

pub struct X86_64 {
    interrupt_descriptor_table: Pin<&'static mut InterruptDescriptorTable>,
}

impl X86_64 {
    pub fn new() -> Self {
        // Initialize the Interrupt Descriptor Table.
        log::info!("Initializing Interrupt Descriptor Table");
        let interrupt_descriptor_table = InterruptDescriptorTable::new();

        // log::info!("Initializing Global Descriptor Table");
        // global_descriptor_table::initialize();
        //
        //
        // // Initialize the PICs.
        // log::info!("Initializing PICs");
        // interrupts::initialize_pics();
        //
        // // Enable interrupts.
        // log::info!("Enabling interrupts");
        // interrupts::enable();
        //
        Self {
            interrupt_descriptor_table,
        }
    }
}

impl ArchitectureApi for X86_64 {}
