use crate::architecture::architecture_api::ArchitectureApi;
use crate::architecture::x86_64::{global_descriptor_table, interrupts};

pub struct X86_64;

impl X86_64 {
    pub fn new() -> Self {
        // Initialize the Global Descriptor Table.
        log::info!("Initializing Global Descriptor Table");
        global_descriptor_table::initialize();

        // Initialize the Interrupt Descriptor Table.
        log::info!("Initializing Interrupt Descriptor Table");
        interrupts::initialize_interrupt_descriptor_table();

        // Initialize the PICs.
        log::info!("Initializing PICs");
        interrupts::initialize_pics();

        // Enable interrupts.
        log::info!("Enabling interrupts");
        interrupts::enable();

        Self
    }
}

impl ArchitectureApi for X86_64 {}
