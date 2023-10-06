use crate::architecture::x86_64::interrupts;

pub fn initialize() {
    // Initialize interrupts.
    interrupts::initialize();

    // Initialize the Interrupt Descriptor Table.

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
}
